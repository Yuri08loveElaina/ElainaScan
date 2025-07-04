use clap::{Parser, ArgGroup};

#[derive(Parser, Debug)]
#[command(author="Yuri08", version, about="ElainaScan - Fast Port & Vuln Scanner")]
#[command(group(
    ArgGroup::new("mode")
        .required(true)
        .args(["banner", "finger", "vuln", "nse"])
))]
pub struct CliArgs {
    #[arg(long)]
    pub target: String,

    #[arg(long)]
    pub ports: String,

    #[arg(long, help = "Banner grab mode")]
    pub banner: bool,

    #[arg(long, help = "OS Fingerprint mode")]
    pub finger: bool,

    #[arg(long, help = "Vulnerability check mode")]
    pub vuln: bool,

    #[arg(long, help = "Run Nmap NSE scripts on target")]
    pub nse: bool,

    #[arg(long, help = "Specify NSE scripts to run (default: vuln)")]
    pub nse_scripts: Option<String>,

    #[arg(long, help = "Save report to file")]
    pub report: Option<String>,

    #[arg(long, help = "Concurrency level")]
    pub concurrency: Option<usize>,
}
