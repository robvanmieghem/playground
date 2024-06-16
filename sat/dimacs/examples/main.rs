extern crate dimacs;

use anyhow::Result;
use clap::Parser;
use std::fs;
use std::io::BufReader;
use std::path::PathBuf;
#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Opt {
    #[arg(default_value = "examples/example.cnf")]
    input_file: PathBuf,
}
fn main() -> Result<()> {
    let args = Opt::parse();
    let input_file = args.input_file.as_path();
    let mut reader = BufReader::new(fs::File::open(input_file)?);
    let mut parser = dimacs::Parser::new(&mut reader);
    let mut m = dimacs::model::Model::new();
    parser.parse_into(&mut m)?;
    println!("{m}");
    Ok(())
}
