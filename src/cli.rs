use clap::{Parser, ArgGroup};

#[derive(Parser, Debug)]
#[command(
    name = "ElainaScan",
    version = "1.0",
    author = "YURI08",
    about = "Rust-based High-Speed CVE & Port Vulnerability Scanner"
)]
#[command(group(
    ArgGroup::new("mode")
        .required(true)
        .args(["banner", "finger", "vuln"])
))]
pub struct CliArgs {
    #[arg(short, long, help = "Target IP or domain (single target)")]
    pub target: String,

    #[arg(short, long, default_value = "80,443", help = "Ports to scan (e.g., 22,80,443 or 20-25)")]
    pub ports: String,

    #[arg(long, help = "Enable Banner Grab Mode")]
    pub banner: bool,

    #[arg(long, help = "Enable Fingerprint (TTL/Window Size) Mode")]
    pub finger: bool,

    #[arg(long, help = "Enable Vulnerability Scan Mode")]
    pub vuln: bool,

    #[arg(long, help = "Output report path (auto-detect JSON, CSV, HTML)")]
    pub report: Option<String>,

    #[arg(long, default_value_t = 100, help = "Concurrency level for scanning")]
    pub concurrency: usize,

    #[arg(long, value_delimiter = ',', help = "Comma-separated NSE scripts to run")]
    pub nse_scripts: Option<Vec<String>>,

    #[arg(long, help = "Path to local CVE database JSON")]
    pub cve_db: Option<String>,

    #[arg(long, default_value = "apache", help = "Service name for CVE checking")]
    pub service: String,

    #[arg(long, default_value = "2.4.49", help = "Service version for CVE checking")]
    pub version: String,
}

impl CliArgs {
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }
}
