
# rusthing

A fast, terminal-native tool for querying and transforming local CSV data files with SQL.

Ingest CSVs, convert to Parquet, run SQL queries via DataFusion, and export results — all from the command line, without spinning up a database.

---


## Features

- **Ingest** — Convert CSV files to Parquet for fast analytics
- **SQL queries** over Parquet files via DataFusion
- **Transformations** — cast types, reshape, filter, aggregate
- **Export** — write results to Parquet
- **Schema inference** — automatically detects column types on load

---


## Usage

```bash
# Ingest a CSV file (creates movies.parquet)
rusthing ingest --csv movies.csv

# Query the Parquet file with SQL
rusthing query --sql "SELECT * FROM movies WHERE year > 2000"

# Example: aggregate
rusthing query --sql "SELECT year, COUNT(*) FROM movies GROUP BY year ORDER BY year"
```

---


## Installation

```bash
cargo install rusthing
```

Or build from source:

```bash
git clone https://github.com/you/rusthing
cd rusthing
cargo build --release
```

---

## Built With

| Crate | Purpose |
|---|---|
| [DataFusion](https://github.com/apache/arrow-datafusion) | SQL query engine |
| [Arrow](https://github.com/apache/arrow-rs) | Columnar data format |
| [Clap](https://github.com/clap-rs/clap) | CLI argument parsing |
| [Tokio](https://tokio.rs) | Async runtime |
| [Serde](https://serde.rs) | Serialisation |

---

## Roadmap

- [ ] CSV and JSON ingestion
- [x] Single-file SQL queries
- [ ] Multi-file joins
- [ ] Type casting and transforms
- [ ] Parquet export
- [ ] Schema inspection command

---

## License

MIT