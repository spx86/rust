
use crate::opts_analysis::opts::{Get, Post, Put, Delete};
use std::collections::HashMap;
use reqwest::{header, Client, Response};
use anyhow::Result;
use colored::*;
use mime::Mime;


pub async fn get(client: Client, args: &Get) -> Result<()>{
    let resp = client.get(&args.url).send().await?;
    Ok(print_resp(resp).await?)
}

pub async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.key, &pair.value);
    }
    let resp = client.post(&args.url).json(&body).send().await?;
    Ok(print_resp(resp).await?)
}

#[warn(unused_variables)]
pub async fn put(client: Client, args: &Put) -> Result<()> {
    Ok(())
}

#[warn(unused_variables)]
pub async fn delete(client: Client, args: &Delete) -> Result<()> {
    Ok(())
}

fn print_status(status: &Response) {
    let status = format!("{:?} {}", status.version(), status.status());
    println!("{}\n", status);
}

fn print_headers(headers: &Response) {
    for (key, value) in headers.headers() {
        println!("{}: {:?}", key.to_string().green(), value);
    }
    println!();
}

fn print_body(m:Option<Mime>, body: &String) {
    match m {
        Some(v) if v == mime::APPLICATION_JSON => {
            println!("{}", jsonxf::pretty_print(body).unwrap().cyan())
        }
        _ => println!("{}", body),
    }
}

fn get_content_type(resp: &Response) -> Option<Mime> {
    resp.headers()
       .get(header::CONTENT_TYPE)
       .map(|v| v.to_str().unwrap().parse().unwrap())
}

async fn print_resp(resp: Response) -> Result<()> {
    print_status(&resp);
    print_headers(&resp);
    let mime = get_content_type(&resp);
    print_body(mime, &resp.text().await?);
    Ok(())
}