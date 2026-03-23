use clap::Parser;
use std::path::PathBuf;
use csv::Reader;

#[derive(Parser, Debug)]
struct Cli {
    csv: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let mut rdr = Reader::from_path(args.csv)?;
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}
