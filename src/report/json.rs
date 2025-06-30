use serde::Serialize;
use std::fs::OpenOptions;
use std::io::Write;
use anyhow::Result;

pub fn write_json_report<T: Serialize>(data: &T, path: &str) -> Result<()> {
    let json = serde_json::to_string_pretty(data)?;
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}
