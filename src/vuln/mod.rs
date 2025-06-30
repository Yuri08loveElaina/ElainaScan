pub mod cve_check;
pub mod nmap_nse;
pub mod exploit;

use anyhow::Result;
use crate::vuln::cve_check::{load_cve_db, check_service_cve};
use crate::vuln::nmap_nse::{run_nse_scan, run_bulk_nse_scan, NseResult};
use crate::vuln::exploit::{run_exploit, ExploitConfig, ExploitResult};

pub fn vuln_check_cve(db_path: &str, service: &str, version: &str) -> Result<()> {
    let cve_db = load_cve_db(db_path)?;
    let result = check_service_cve(service, version, &cve_db);
    match result {
        Ok(list) => {
            for cve in list {
                println!("{}", cve);
            }
        },
        Err(_) => println!("No CVE found for {} {}", service, version),
    }
    Ok(())
}

pub fn vuln_scan_nse_single(ip: &str, port: u16, scripts: &[&str]) -> Result<Vec<NseResult>> {
    let results = run_nse_scan(ip, port, scripts, Some("nse_results.json"))?;
    Ok(results)
}

pub fn vuln_scan_nse_bulk(targets: &[(String, u16)], scripts: &[&str], concurrency: usize) -> Result<()> {
    run_bulk_nse_scan(targets, scripts, concurrency, Some("nse_results.json"))?;
    Ok(())
}

pub fn vuln_run_exploit(ip: &str, port: u16, exploit_path: &str, additional_args: &[&str]) -> Result<ExploitResult> {
    let config = ExploitConfig {
        exploit_path,
        args: additional_args.to_vec(),
        timeout: 30,
        retries: 2,
    };
    let result = run_exploit(ip, port, &config)?;
    println!("[EXPLOIT] {}:{} -> {}", ip, port, result.status);
    Ok(result)
}
