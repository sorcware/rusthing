# rusthing

A fast, terminal-native tool for querying and transforming local data files with SQL.

Load CSVs and JSON files, run SQL against them, reshape and join datasets, and export results — all from the command line, without spinning up a database.

---

## Features

- **SQL queries** over CSV and JSON files via DataFusion
- **Multi-file joins** — treat multiple files as tables in the same query
- **Transformations** — cast types, reshape, filter, aggregate
- **Export** — write results to CSV or Parquet
- **Schema inference** — automatically detects column types on load

---

## Usage

```bash
# Query a single file
rusthing query --file data.csv "SELECT region, SUM(revenue) FROM data GROUP BY region"

# Join two files
rusthing query --file orders.csv --file customers.csv \
  "SELECT c.name, SUM(o.total) FROM orders o JOIN customers c ON o.customer_id = c.id GROUP BY c.name"

# Export results
rusthing query --file data.csv --out results.parquet "SELECT * FROM data WHERE year = 2024"
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
- [ ] Single-file SQL queries
- [ ] Multi-file joins
- [ ] Type casting and transforms
- [ ] Parquet export
- [ ] Schema inspection command

---

## License

MIT