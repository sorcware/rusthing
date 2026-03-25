use csv::Reader;
use arrow::array::{StringBuilder, UInt16Builder, ArrayRef};
use arrow::record_batch::RecordBatch;
use arrow::datatypes::{DataType, Field, Schema};
use std::sync::Arc;
use parquet::arrow::ArrowWriter;
use parquet::file::properties::WriterProperties;
use datafusion::prelude::*;
use std::path::PathBuf;

#[derive(Debug)]
struct Movie {
    title: String,
    year: u16,
    runtime_minutes: u16,
}

pub async fn ingest(input_csv_path: PathBuf, output_parquet_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut rdr = Reader::from_path(input_csv_path)?;
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
    let file = std::fs::File::create(output_parquet_path)?;
    let mut writer = ArrowWriter::try_new(file, batch.schema(), Some(props))?;
    writer.write(&batch)?;
    writer.close()?;
    Ok(())
}

pub async fn sqlquery(sql: String, parquet_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let ctx = SessionContext::new();
    ctx.register_parquet("movies", parquet_path.to_str().unwrap(), ParquetReadOptions::default()).await?;
    let df = ctx.sql(&sql).await?;
    let results: Vec<RecordBatch> = df.collect().await?;
    let pretty_results = arrow::util::pretty::pretty_format_batches(&results)?.to_string();
    println!("{}", pretty_results);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[tokio::test]
    async fn test_ingest_creates_parquet() {
        let csv_path = "test_movies.csv";
        let parquet_path = "test_movies.parquet";
        let csv_content = "title,year,runtime_minutes\ntest,2020,100\n";
        fs::write(csv_path, csv_content).unwrap();

        ingest(csv_path.into(), parquet_path.into()).await.unwrap();

        let metadata = fs::metadata(parquet_path).unwrap();
        assert!(metadata.len() > 0, "Parquet file should not be empty");

        let _ = fs::remove_file(csv_path);
        let _ = fs::remove_file(parquet_path);
    }
}
