use clap::{Parser, Subcommand};
use std::path::PathBuf;
use rusthing::{ingest, sqlquery};


 #[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Ingest{
        #[arg(short, long)]
        csv: PathBuf,
        #[arg(short, long)]
        parquet: PathBuf,
    },
    Query{
        #[arg(short, long)]
        sql: String,
        #[arg(short, long)]
        parquet: PathBuf,
    },
}

 #[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    println!("Running command: {:?}", args.command);
    match args.command {
        Command::Ingest { csv, parquet } => ingest(csv, parquet).await?,
        Command::Query { sql, parquet } => sqlquery(sql, parquet).await?,
    }
    Ok(())
}