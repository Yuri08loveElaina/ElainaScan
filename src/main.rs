mod cli;
mod nse;
mod finger;
mod banner;
mod vuln;
mod report;
mod utils;
use utils::banner;

fn main() -> anyhow::Result<()> {
    banner::show();
    // CLI + scan logic tiếp theo
    Ok(())
}
use cli::{CliArgs, FingerMode};
use anyhow::Result;

fn main() -> Result<()> {
    let args = CliArgs::parse();

    if args.debug {
        println!("[DEBUG] Parsed arguments: {:#?}", args);
    }

    if args.banner {
        println!("[*] Running Banner Grab on {}:{}", args.target, args.ports);
        banner::banner::run_banner_scan(&args.target, &args.ports, args.timeout.unwrap_or(1000), args.report.as_deref())?;
    }

    if args.finger {
        match args.finger_mode {
            Some(FingerMode::Fast) => {
                println!("[*] Running Fast Fingerprint (Nmap -O -sV)...");
                finger::finger::run_finger_nmap(&args.target, &args.ports)?;
            },
            Some(FingerMode::Stealth) | None => {
                println!("[*] Running Stealth Fingerprint (Rust TTL + Banner)...");
                finger::finger::run_finger_stealth(&args.target, &args.ports, args.timeout.unwrap_or(1000))?;
            },
        }
    }

    if args.vuln {
        println!("[*] Running Local CVE Vuln Scan on {}", args.target);
        vuln::cve_check::run_local_cve_scan(&args.target, &args.ports, args.report.as_deref())?;
    }

    if args.nse {
        println!("[*] Running Nmap NSE scripts on {}", args.target);
        nse::nse::run_nmap_nse(
            &args.target,
            &args.ports,
            args.nse_scripts.as_deref(),
            args.report.as_deref()
        )?;
    }

    println!("\x1b[94m[*] ElainaScan completed.\x1b[0m");

    Ok(())
}
