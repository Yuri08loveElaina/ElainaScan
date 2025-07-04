use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(
    name = "ElainaScan",
    version = "0.1.0",
    about = "🌸 Fast, aesthetic Rust-based Port + Vuln Scanner",
    author = "Yuri08loveElaina"
)]
pub struct CliArgs {
    /// Target IP or domain
    #[arg(short, long, help = "Target IP or domain")]
    pub target: String,

    /// Ports to scan (e.g., 22,80,443 or 1-65535)
    #[arg(short, long, help = "Ports to scan (e.g., 22,80,443 or 1-65535)")]
    pub ports: String,

    /// Enable Banner Grab Mode
    #[arg(long, help = "Enable Banner Grab Mode")]
    pub banner: bool,

    /// Enable Fingerprint Mode
    #[arg(long, help = "Enable Fingerprint Mode")]
    pub finger: bool,

    /// Fingerprint mode type (fast using Nmap or stealth using Rust)
    #[arg(long, value_enum, help = "Fingerprint mode type (fast or stealth)")]
    pub finger_mode: Option<FingerMode>,

    /// Enable Vulnerability Check
    #[arg(long, help = "Enable Vulnerability Scan using local CVE DB")]
    pub vuln: bool,

    /// Enable Nmap NSE Script Scan
    #[arg(long, help = "Enable Nmap NSE Script Scan")]
    pub nse: bool,

    /// Custom NSE scripts (comma-separated)
    #[arg(long, help = "Custom NSE scripts to run (comma-separated)")]
    pub nse_scripts: Option<String>,

    /// Output report file (JSON, TXT, CSV, HTML)
    #[arg(short, long, help = "Output report file (JSON, TXT, CSV, HTML)")]
    pub report: Option<String>,

    /// Set concurrency for port scan
    #[arg(long, help = "Concurrency level for scanning (default 100)")]
    pub concurrency: Option<usize>,

    /// Set timeout in ms
    #[arg(long, help = "Timeout per connection in milliseconds")]
    pub timeout: Option<u64>,

    /// Silent mode (less verbose)
    #[arg(long, help = "Silent mode (less verbose)")]
    pub silent: bool,

    /// Debug mode (show arguments and debug info)
    #[arg(long, help = "Enable debug mode")]
    pub debug: bool,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum FingerMode {
    Fast,
    Stealth,
}

impl CliArgs {
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }
}
