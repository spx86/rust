
mod opts_analysis;
mod http_request;
use clap::Parser;
use opts_analysis::opts::{Opts};
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

    let req = make_request(client, &args).await?;
    Ok(req)
}
