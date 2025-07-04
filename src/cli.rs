use clap::{Parser, ArgGroup, ValueEnum};

#[derive(Parser, Debug)]
#[command(
    author = "Yuri08",
    version,
    about = "ElainaScan - Fast Port & Vuln Scanner",
    long_about = "🌸 ElainaScan là công cụ pentest quét port + vuln siêu nhanh bằng Rust, hỗ trợ NSE Nmap auto."
)]
#[command(group(
    ArgGroup::new("mode")
        .required(true)
        .args(["banner", "finger", "vuln", "nse"])
))]
pub struct CliArgs {
    /// Target IP or domain (e.g., 192.168.1.10, google.com)
    #[arg(long)]
    pub target: String,

    /// Comma-separated ports or range (e.g., 22,80,443 or 1-1000)
    #[arg(long)]
    pub ports: String,

    /// Enable banner grabbing on open ports
    #[arg(long, help = "Banner grab mode")]
    pub banner: bool,

    /// Enable OS fingerprinting on target
    #[arg(long, help = "OS Fingerprint mode")]
    pub finger: bool,

    /// Enable vulnerability scan using local CVE database
    #[arg(long, help = "Vulnerability check mode")]
    pub vuln: bool,

    /// Run Nmap NSE scripts on target
    #[arg(long, help = "Run Nmap NSE scripts on target")]
    pub nse: bool,

    /// Specify NSE scripts to run (default: vuln)
    #[arg(long, help = "Specify NSE scripts to run, e.g., http-vuln-cve2006-3392")]
    pub nse_scripts: Option<String>,

    /// Save scan results to a report file
    #[arg(long, help = "Save report to JSON/CSV/HTML file")]
    pub report: Option<String>,

    /// Set concurrency level for scanning
    #[arg(long, help = "Concurrency level for port scan (default: 1000)")]
    pub concurrency: Option<usize>,

    /// Timeout per connection in milliseconds
    #[arg(long, help = "Timeout per probe in milliseconds")]
    pub timeout: Option<u64>,

    /// Enable debug output for troubleshooting
    #[arg(long, help = "Enable verbose debug output")]
    pub debug: bool,

    /// Silence non-essential output, only print results
    #[arg(long, help = "Silent mode, only print results")]
    pub silent: bool,
}
