
use clap::{Parser, Subcommand};
use super::format_vaild::*;
//{parse_url, KVPair};

#[derive(Parser, Debug)]
pub struct Opts {
    #[command(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    Get(Get),
    Post(Post),
    Put(Put),
    Delete(Delete),
}

#[derive(Parser, Debug)]
pub struct Get {
    #[clap(value_parser(parse_url))]
    url: String,
}

#[derive(Parser, Debug)]
pub struct Post {
    #[clap(value_parser(parse_url))]
    url: String,
    #[clap(value_parser(parse_kv_pairs))]
    body: Vec<KVPair>,
}

#[derive(Parser, Debug)]
pub struct Put {
    #[clap(value_parser(parse_url))]
    url: String,

    body: Vec<KVPair>,
}

#[derive(Parser, Debug)]
pub struct Delete {
    #[clap(value_parser(parse_url))]
    url: String,
}
