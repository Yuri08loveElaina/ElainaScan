use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use std::time::Duration;
use anyhow::Result;

pub async fn grab_ftp(ip: &str, port: u16) -> Result<()> {
    let addr = format!("{}:{}", ip, port);
    let mut stream = tokio::time::timeout(Duration::from_secs(2), TcpStream::connect(&addr)).await??;
    let mut buf = [0u8; 256];
    let n = stream.read(&mut buf).await?;
    let banner = String::from_utf8_lossy(&buf[..n]).trim().to_string();
    println!("[FTP] {}:{} -> {}", ip, port, banner);
    stream.write_all(b"QUIT\r\n").await?;
    Ok(())
}
