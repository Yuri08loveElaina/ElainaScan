pub mod http;
pub mod ssh;
pub mod ftp;

use crate::utils::parse_ip_port;
use colored::*;
use futures::stream::{FuturesUnordered, StreamExt};

pub async fn grab_banners(ip_port_list: Vec<String>, concurrency: usize) {
    let mut tasks = FuturesUnordered::new();

    for ip_port in ip_port_list {
        let task = tokio::spawn(async move {
            if let Err(e) = grab_banner_single(ip_port.clone()).await {
                println!("{}", format!("[BANNER] {} -> Error: {}", ip_port, e).red());
            }
        });

        tasks.push(task);

        if tasks.len() >= concurrency {
            let _ = tasks.next().await;
        }
    }

    while let Some(_) = tasks.next().await {}
}

async fn grab_banner_single(ip_port: String) -> anyhow::Result<()> {
    let (ip, port) = parse_ip_port(&ip_port);

    match port {
        21 => {
            let banner = ftp::grab_ftp(&ip, port).await?;
            println!("{}", format!("[FTP] {}:{} -> {}", ip, port, banner).blue());
        }
        22 => {
            let banner = ssh::grab_ssh(&ip, port).await?;
            println!("{}", format!("[SSH] {}:{} -> {}", ip, port, banner).yellow());
        }
        80 | 443 | 8000 | 8080 | 8888 => {
            let banner = http::grab_http(&ip, port).await?;
            println!("{}", format!("[HTTP] {}:{} -> {}", ip, port, banner).cyan());
        }
        _ => {}
    }

    Ok(())
}
