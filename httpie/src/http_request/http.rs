
use crate::opts_analysis::opts::{Opts, SubCommand};
use crate::opts_analysis::format_vaild::KVPair;
use std::collections::HashMap;
use reqwest::{header, Client, Response};
use anyhow::Result;
use colored::*;
use mime::Mime;


// pub async fn get(client: Client, args: &Get) -> Result<()>{
//     let resp = client.get(&args.url).send().await?;
//     Ok(print_resp(resp).await?)
// }

// pub async fn post(client: Client, args: &Post) -> Result<()> {
//     let mut body = HashMap::new();
//     for pair in args.body.iter() {
//         body.insert(&pair.key, &pair.value);
//     }
//     let resp = client.post(&args.url).json(&body).send().await?;
//     Ok(print_resp(resp).await?)
// }

// pub async fn put(client: Client, args: &Put) -> Result<()> {
//     let mut body = HashMap::new();
//     for pair in args.body.iter() {
//         body.insert(&pair.key, &pair.value);
//     }
//     let resp = client.put(&args.url).json(&body).send().await?;
//     Ok(print_resp(resp).await?)
// }
// pub async fn delete(client: Client, args: &Delete) -> Result<()> {
//     let resp = client.delete(&args.url).send().await?;
//     Ok(print_resp(resp).await?)
// }


// 请求的主逻辑
pub async fn make_request(client: Client, args: &Opts) -> Result<()> {
    // 根据SubCommand构建URL
    let (url, body) = match &args.subcmd {
        SubCommand::Get(get) => (get.url.clone(), build_body(&get.body)),
        SubCommand::Post(post) => (post.url.clone(), build_body(&post.body)),
        SubCommand::Put(put) => (put.url.clone(), build_body(&put.body)),
        SubCommand::Delete(delete) => (delete.url.clone(), build_body(&delete.body)),
    };

    // 构建并发送请求
    let resp = match &args.subcmd {
        SubCommand::Get(_) => client.get(&url).send().await?, // GET 不传body
        SubCommand::Post(_) | SubCommand::Put(_) => client.post(&url).json(&body).send().await?, // POST 和 PUT 可以传 body
        SubCommand::Delete(_) => client.delete(&url).send().await?, // DELETE 也不传body
    };

    // 处理响应
    Ok(print_resp(resp).await?)
}

// 处理键值对列表，构建请求体
fn build_body(pairs: &Vec<KVPair>) -> HashMap<String, String> {
    let mut body = HashMap::new();
    for pair in pairs.iter() {
        body.insert(pair.key.clone(), pair.value.clone());
    }
    body
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