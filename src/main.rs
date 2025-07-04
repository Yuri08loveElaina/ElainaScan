mod cli;
mod nse;

use cli::CliArgs;
use anyhow::Result;

fn main() -> Result<()> {
    let args = CliArgs::parse();

    // Banner Grab Mode (placeholder)
    if args.banner {
        println!("[*] Banner grab mode selected for target: {}", args.target);
        println!("(Tính năng này sẽ được triển khai ở module banner)");
    }

    // Fingerprint Mode (placeholder)
    if args.finger {
        println!("[*] OS Fingerprinting mode selected for target: {}", args.target);
        println!("(Tính năng này sẽ được triển khai ở module finger)");
    }

    // Vulnerability Scan Mode (placeholder)
    if args.vuln {
        println!("[*] Vulnerability scan mode selected for target: {}", args.target);
        println!("(Tính năng này sẽ được triển khai ở module vuln với local CVE DB)");
    }

    // Nmap NSE Mode
    if args.nse {
        if args.silent == false {
            println!("[*] Running Nmap NSE scripts on target: {}", args.target);
            if let Some(scripts) = &args.nse_scripts {
                println!("[*] Using custom NSE scripts: {}", scripts);
            } else {
                println!("[*] Using default NSE script: vuln");
            }
        }

        nse::nse::run_nmap_nse(&args.target, &args.ports, args.nse_scripts.as_deref())?;
    }

    Ok(())
}
