mod cli;
mod scanner;
mod utils;

use clap::Parser;
use cli::Args;
use utils::parse_ports;
use scanner::scan_ports;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let ports = parse_ports(&args.ports);

    println!("🚀 Starting ElainaScan on {}", args.target);
    scan_ports(&args.target, ports, args.concurrency).await;
}
