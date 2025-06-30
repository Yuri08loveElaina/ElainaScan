use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use std::time::Duration;
use anyhow::Result;

pub async fn grab_http(ip: &str, port: u16) -> Result<()> {
    let addr = format!("{}:{}", ip, port);
    let stream = tokio::time::timeout(Duration::from_secs(2), TcpStream::connect(&addr)).await??;
    let (mut reader, mut writer) = stream.into_split();

    writer.write_all(b"HEAD / HTTP/1.0\r\n\r\n").await?;
    let mut buf = [0u8; 1024];
    let n = reader.read(&mut buf).await?;
    let resp = String::from_utf8_lossy(&buf[..n]);
    let line = resp.lines().find(|l| l.to_lowercase().starts_with("server"))
        .unwrap_or_else(|| resp.lines().next().unwrap_or("No banner"));
    println!("[HTTP] {}:{} -> {}", ip, port, line);
    Ok(())
}
