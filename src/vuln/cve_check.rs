use anyhow::{Result, anyhow};
use serde::{Deserialize};
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct CVEEntry {
    pub cve_id: String,
    pub description: String,
    pub affected_products: Vec<String>,
}

pub fn load_cve_db(path: &str) -> Result<Vec<CVEEntry>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let entries: Vec<CVEEntry> = serde_json::from_reader(reader)?;
    Ok(entries)
}

pub fn check_service_cve(service: &str, version: &str, cve_db: &[CVEEntry]) -> Result<Vec<String>> {
    let mut found = Vec::new();
    let target = format!("{} {}", service.to_lowercase(), version.to_lowercase());

    for entry in cve_db {
        for affected in &entry.affected_products {
            if target.contains(&affected.to_lowercase()) {
                found.push(format!(
                    "{} | {}",
                    entry.cve_id,
                    entry.description.chars().take(80).collect::<String>()
                ));
            }
        }
    }

    if found.is_empty() {
        Err(anyhow!("No CVE found"))
    } else {
        Ok(found)
    }
}
