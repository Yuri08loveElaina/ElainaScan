use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "ElainaScan - Fast, Precise, Beautiful")]
pub struct Args {
    /// Target IP or CIDR
    #[arg(short, long)]
    pub target: String,

    /// Port range (e.g., 1-65535)
    #[arg(short, long, default_value = "1-1024")]
    pub ports: String,

    /// Concurrency limit
    #[arg(short, long, default_value = "1000")]
    pub concurrency: usize,
}
