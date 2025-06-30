use serde::Serialize;
use std::fs::File;
use std::io::Writer;
use anyhow::Result;
use csv::WriterBuilder;

pub fn write_csv_report<T: Serialize>(data: &[T], path: &str) -> Result<()> {
    let file = File::create(path)?;
    let mut wtr = WriterBuilder::new()
        .has_headers(true)
        .from_writer(file);
    for record in data {
        wtr.serialize(record)?;
    }
    wtr.flush()?;
    Ok(())
}
