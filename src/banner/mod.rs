pub mod http;
pub mod ssh;
pub mod ftp;

use crate::utils::parse_ip_port;
use colored::*;
use futures::stream::{FuturesUnordered, StreamExt};

pub async fn grab_banners(ip_port_list: Vec<String>, concurrency: usize) {
    let mut tasks = FuturesUnordered::new();

    for ip_port in ip_port_list {
        let task = tokio::spawn(grab_banner_single(ip_port));
        tasks.push(task);

        if tasks.len() >= concurrency {
            let _ = tasks.next().await;
        }
    }

    while let Some(_) = tasks.next().await {}
}

async fn grab_banner_single(ip_port: String) {
    let (ip, port) = parse_ip_port(&ip_port);
    match port {
        21 => {
            if let Err(e) = ftp::grab_ftp(&ip, port).await {
                println!("{}", format!("[FTP] {}:{} grab error: {}", ip, port, e).red());
            }
        }
        22 => {
            if let Err(e) = ssh::grab_ssh(&ip, port).await {
                println!("{}", format!("[SSH] {}:{} grab error: {}", ip, port, e).red());
            }
        }
        80 | 443 | 8000 | 8080 | 8888 => {
            if let Err(e) = http::grab_http(&ip, port).await {
                println!("{}", format!("[HTTP] {}:{} grab error: {}", ip, port, e).red());
            }
        }
        _ => {}
    }
}
