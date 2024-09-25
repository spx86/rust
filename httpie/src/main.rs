
mod opts_analysis;
use clap::Parser;
use opts_analysis::opts::Opts;
fn main() {
    let args = Opts::parse();
    println!("{args:#?}");
}
