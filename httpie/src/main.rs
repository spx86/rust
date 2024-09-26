
mod opts_analysis;
mod http_request;
use clap::Parser;
use opts_analysis::opts::{Opts, SubCommand};
use reqwest::{header, Client};
use anyhow::Result;
use http_request::http::*;

#[tokio::main]
async fn main() -> Result<()>{
    let args = Opts::parse();

    let mut headers = header::HeaderMap::new();
    headers.insert("X-POWERED-BY", "Rust".parse()?);
    headers.insert(header::USER_AGENT, "Rust Httpie".parse()?);

    let client = Client::builder()
        .default_headers(headers)
        .build()?;

    let req = match args.subcmd {
        SubCommand::Get(ref args) => get(client, args).await?,
        SubCommand::Post(ref args) => post(client, args).await?,
        SubCommand::Put(ref args) => put(client, args).await?,
        SubCommand::Delete(ref args) => delete(client, args).await?,
    };
    Ok(req)
}
