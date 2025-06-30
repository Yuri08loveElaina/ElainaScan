pub mod json;
pub mod csv;
pub mod html;

use anyhow::Result;
use serde::Serialize;

pub fn report_json<T: Serialize>(data: &T, path: &str) -> Result<()> {
    json::write_json_report(data, path)?;
    println!("[REPORT] JSON saved to {}", path);
    Ok(())
}

pub fn report_csv<T: Serialize>(data: &[T], path: &str) -> Result<()> {
    csv::write_csv_report(data, path)?;
    println!("[REPORT] CSV saved to {}", path);
    Ok(())
}

pub fn report_html<T: Serialize>(data: &[T], path: &str) -> Result<()> {
    html::write_html_report(data, path)?;
    println!("[REPORT] HTML saved to {}", path);
    Ok(())
}
