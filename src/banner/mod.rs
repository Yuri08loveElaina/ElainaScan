pub mod http;
pub mod ssh;
pub mod ftp;

use crate::utils::parse_ip_port;

pub async fn grab_banner(ip_port: &str) {
    let (ip, port) = parse_ip_port(ip_port);
    match port {
        80 | 8080 | 8000 => {
            http::grab_http(&ip, port).await;
        }
        22 => {
            ssh::grab_ssh(&ip, port).await;
        }
        21 => {
            ftp::grab_ftp(&ip, port).await;
        }
        _ => {}
    }
}
