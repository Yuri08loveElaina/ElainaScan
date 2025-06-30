pub mod ttl;
pub mod win_size;

use crate::utils::parse_ip_port;
use colored::*;
use futures::stream::{FuturesUnordered, StreamExt};

pub async fn fingerprint_targets(ip_port_list: Vec<String>, concurrency: usize) {
    let mut tasks = FuturesUnordered::new();

    for ip_port in ip_port_list {
        let task = tokio::spawn(fingerprint_single(ip_port));
        tasks.push(task);

        if tasks.len() >= concurrency {
            let _ = tasks.next().await;
        }
    }

    while let Some(_) = tasks.next().await {}
}

async fn fingerprint_single(ip_port: String) {
    let (ip, port) = parse_ip_port(&ip_port);

    let ttl_result = ttl::fingerprint_ttl(&ip).await;
    match ttl_result {
        Ok(ttl_value) => {
            let os_guess = match ttl_value {
                0..=64 => "Possible Linux/Unix".green(),
                65..=128 => "Possible Windows".yellow(),
                129..=255 => "Possible Cisco/FreeBSD".cyan(),
                _ => "Unknown OS".white(),
            };
            println!("[FP] {} -> TTL={} => {}", ip, ttl_value, os_guess);
        }
        Err(e) => {
            println!("[FP] {} -> TTL grab error: {}", ip, e);
        }
    }

    let win_result = win_size::fingerprint_win_size(&ip, port).await;
    match win_result {
        Ok(win_size) => {
            let os_guess = match win_size {
                8192 | 65535 => "Possible Windows".yellow(),
                29200 | 5840 | 14600 => "Possible Linux".green(),
                _ => "Unknown OS".white(),
            };
            println!("[FP] {}:{} -> WinSize={} => {}", ip, port, win_size, os_guess);
        }
        Err(e) => {
            println!("[FP] {}:{} -> WinSize grab error: {}", ip, port, e);
        }
    }
}
