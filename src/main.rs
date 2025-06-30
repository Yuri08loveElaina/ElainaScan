mod banner;
mod finger;
mod vuln;
mod report;
mod utils;
mod cli;

use anyhow::Result;
use cli::CliArgs;

fn main() -> Result<()> {
    let args = CliArgs::parse();

    if args.banner {
        let results = banner::mod_banner_scan(&args.target, &args.ports)?;
        if let Some(path) = &args.report {
            report::report_json(&results, path)?;
        }
    }

    if args.finger {
        let results = finger::mod_ttl_finger(&args.target)?;
        if let Some(path) = &args.report {
            report::report_json(&results, path)?;
        }
    }

    if args.vuln {
        if let Some(cve_db) = &args.cve_db {
            vuln::vuln_check_cve(cve_db, &args.service, &args.version)?;
        }
        if let Some(scripts) = &args.nse_scripts {
            let targets = utils::parse_targets(&args.target, &args.ports)?;
            vuln::vuln_scan_nse_bulk(&targets, scripts, args.concurrency)?;
        }
    }

    Ok(())
}
