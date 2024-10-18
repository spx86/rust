use crate::DataSet;
use anyhow::Result;
use polars::{io::csv, prelude::*};
use std::{fmt::Result, io::Cursor};

pub trait Load {
    type Error;
    fn load(self) -> Result<DataSet, Self::Error>;
}

#[derive(Debug)]
#[non_exhaustive]
pub enum Loader {
    Csv(Csvloader),
}

#[derive(Debug, Default)]
pub struct Csvloader(pub(crate) String);

impl Loader{
    pub fn load(self) -> Result<DataSet> {
        match self {
            Loader::Csv(csv) => csv.load(),
        }
    }
}

pub fn detect_content(data: String) -> Loader {
    Loader::Csv(Csvloader(data))
}

impl Load for Csvloader {
    type Error = anyhow::Error;
    
    fn load(self) -> Result<DataSet, Self::Error> {
        let df = CsvReader::new(Cursor::new(self.0))
            .infer_schema(Some(16))
            .finish()?;
        Ok(DataSet(df))
    }
}