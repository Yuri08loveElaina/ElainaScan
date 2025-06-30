use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::collections::HashSet;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CVEEntry {
    pub cve_id: String,
    pub description: String,
    pub affected_products: Vec<String>,
    pub severity: Option<String>,
}

pub fn load_cve_db(path: &str) -> Result<Vec<CVEEntry>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let entries: Vec<CVEEntry> = serde_json::from_reader(reader)?;
    Ok(entries)
}

pub fn check_service_cve(service: &str, version: &str, cve_db: &[CVEEntry]) -> Result<Vec<String>> {
    let mut found = HashSet::new();
    let normalized_service = service.to_lowercase();
    let normalized_version = version.to_lowercase();

    for entry in cve_db {
        for affected in &entry.affected_products {
            let affected_normalized = affected.to_lowercase();
            if affected_normalized.contains(&normalized_service) && affected_normalized.contains(&normalized_version) {
                let severity = entry.severity.clone().unwrap_or_else(|| "Unknown".to_string());
                let desc = entry.description.chars().take(100).collect::<String>();
                found.insert(format!(
                    "{} | Severity: {} | {}",
                    entry.cve_id,
                    severity,
                    desc
                ));
            }
        }
    }

    if found.is_empty() {
        Err(anyhow!("No CVE found for {} {}", service, version))
    } else {
        Ok(found.into_iter().collect())
    }
}
