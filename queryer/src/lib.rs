use anyhow::{anyhow, Ok, Result};
use polars::prelude::*;
use sqlparser::parser::Parser;
use std::convert::TryInto;
use std::ops::{Deref, DerefMut};
use tracing::info;

mod dialect;
mod convert;
mod fetcher;
mod loader;

use convert::Sql;
use loader::detect_content;
use fetcher::retrieve_data;

pub use dialect::TyrDialect;

#[derive(Debug)]
pub struct DataSet(DataFrame);

impl Deref for DataSet {
    type Target = DataFrame;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DataSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DataSet {
    pub fn to_csv(&mut self) -> Result<String> {
        let mut buf = Vec::new();
        let mut writer = CsvWriter::new(&mut buf);
        writer.finish(&mut self.0)?;  // 使用 &mut self.0 传递底层 DataFrame
        Ok(String::from_utf8(buf)?)
    }
}

pub async fn query<T: AsRef<str>>(sql: T) -> Result<DataSet> {
    let ast = Parser::parse_sql(&TyrDialect::default(), sql.as_ref())?;

    if ast.len() != 1 {
        return Err(anyhow!("Expected a single SQL statement"));
    }

    let sql = &ast[0];

    let Sql {
        source,
        condition,
        selection,
        offset,
        limit,
        order_by,
    } = sql.try_into()?;

    info!("retrieving data from source: {}", source);

    let ds = detect_content(retrieve_data(source).await?).load()?;

    let mut filtered = match condition {
        Some(expr) => ds.0.lazy().filter(expr),
        None => ds.0.lazy(),
    };

    filtered = order_by
                .into_iter()
                .fold(filtered, |acc, (col, desc)| 
                    acc.sort(vec![col].into_vec(), SortMultipleOptions::new()
                .with_order_descending(desc)));

    if offset.is_some() || limit.is_some() {
        filtered = filtered.slice(
            offset.unwrap_or(0),
            limit.unwrap_or(usize::MAX).try_into().unwrap_or(u32::MAX)
        );
    }

    Ok(DataSet(filtered.select(selection).collect()?))
}
