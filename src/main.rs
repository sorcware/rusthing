use clap::Parser;
use std::path::PathBuf;
use csv::Reader;
use arrow::array::{StringBuilder, UInt16Builder, ArrayRef};
use arrow::record_batch::RecordBatch;
use arrow::datatypes::{DataType, Field, Schema};
use std::sync::Arc;
use parquet::arrow::ArrowWriter;
use parquet::file::properties::WriterProperties;

#[derive(Parser, Debug)]
struct Cli {
    csv: PathBuf,
}

#[derive(Debug)]
struct Movie {
    title: String,
    year: u16,
    runtime_minutes: u16,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let mut rdr = Reader::from_path(args.csv)?;
    let mut movies: Vec<Movie> = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let movie = Movie {
            title: record[0].to_string(),
            year: record[1].parse()?,
            runtime_minutes: record[2].parse()?,
        };
        movies.push(movie);
    }

    let mut title_builder = StringBuilder::new();
    for movie in &movies {
        title_builder.append_value(&movie.title);
    }
    let title_array = Arc::new(title_builder.finish()) as ArrayRef;

    let mut year_builder = UInt16Builder::new();
    for movie in &movies {
        year_builder.append_value(movie.year);
    }
    let year_array = Arc::new(year_builder.finish()) as ArrayRef;

    let mut runtime_builder = UInt16Builder::new();
    for movie in &movies {
        runtime_builder.append_value(movie.runtime_minutes);
    }
    let runtime_array = Arc::new(runtime_builder.finish()) as ArrayRef;

    let schema = Arc::new(Schema::new(vec![
        Field::new("title", DataType::Utf8, false),
        Field::new("year", DataType::UInt16, false),
        Field::new("runtime_minutes", DataType::UInt16, false),
    ]));

    let batch = RecordBatch::try_new(
        schema,
        vec![title_array, year_array, runtime_array],
    )?;

    let props = WriterProperties::builder().build();
    let file = std::fs::File::create("movies.parquet")?;
    let mut writer = ArrowWriter::try_new(file, batch.schema(), Some(props))?;
    writer.write(&batch)?;
    writer.close()?;
    Ok(())
}

