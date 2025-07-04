pub mod ttl;
pub mod win_size;
pub mod finger;

use crate::utils::parse_ip_port;
use colored::*;
use futures::stream::{FuturesUnordered, StreamExt};

pub async fn fingerprint_targets(ip_port_list: Vec<String>, concurrency: usize) {
    let mut tasks = FuturesUnordered::new();

    for ip_port in ip_port_list {
        let task = tokio::spawn(async move {
            if let Err(e) = fingerprint_single(ip_port.clone()).await {
                println!("{}", format!("[FP] {} -> Error: {}", ip_port, e).red());
            }
        });

        tasks.push(task);

        if tasks.len() >= concurrency {
            let _ = tasks.next().await;
        }
    }

    while let Some(_) = tasks.next().await {}
}

async fn fingerprint_single(ip_port: String) -> anyhow::Result<()> {
    let (ip, port) = parse_ip_port(&ip_port);

    let ttl_value = ttl::fingerprint_ttl(&ip).await.unwrap_or(0);
    let ttl_guess = match ttl_value {
        1..=64 => "Linux/Unix".green(),
        65..=128 => "Windows".yellow(),
        129..=255 => "Cisco/FreeBSD".cyan(),
        _ => "Unknown".white(),
    };

    if ttl_value > 0 {
        println!(
            "{}",
            format!("[FP] {} -> TTL={} => {}", ip, ttl_value, ttl_guess)
        );
    } else {
        println!("{}", format!("[FP] {} -> TTL grab failed", ip).red());
    }

    let win_size = win_size::fingerprint_win_size(&ip, port).await.unwrap_or(0);
    let win_guess = match win_size {
        8192 | 65535 => "Windows".yellow(),
        29200 | 5840 | 14600 => "Linux".green(),
        _ => "Unknown".white(),
    };

    if win_size > 0 {
        println!(
            "{}",
            format!("[FP] {}:{} -> WinSize={} => {}", ip, port, win_size, win_guess)
        );
    } else {
        println!(
            "{}",
            format!("[FP] {}:{} -> WinSize grab failed", ip, port).red()
        );
    }

    Ok(())
}
