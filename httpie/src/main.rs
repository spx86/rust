
mod opts_analysis;
use clap::Parser;
use opts_analysis::opts::Opts;
use opts_analysis::format_vaild::*;
fn main() {
    let args = Opts::parse();
    println!("{args:#?}");
}
