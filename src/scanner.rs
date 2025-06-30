use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use futures::stream::{FuturesUnordered, StreamExt};
use colored::*;

pub async fn scan_ports(target: &str, ports: Vec<u16>, concurrency: usize) {
    let mut tasks = FuturesUnordered::new();

    for port in ports {
        let addr = format!("{}:{}", target, port);
        let task = tokio::spawn(async move {
            if let Ok(_) = timeout(Duration::from_secs(1), TcpStream::connect(&addr)).await {
                println!("{}", format!("[OPEN] {}:{}", target, port).green());
            }
        });
        tasks.push(task);

        if tasks.len() >= concurrency {
            tasks.next().await;
        }
    }

    while let Some(_) = tasks.next().await {}
}
