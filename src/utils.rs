use anyhow::{Result, bail};
use std::net::IpAddr;
use std::str::FromStr;

pub fn parse_ports(port_range: &str) -> Result<Vec<u16>> {
    let mut ports = Vec::new();
    for part in port_range.split(',') {
        if let Some((start, end)) = part.split_once('-') {
            let start = start.trim().parse::<u16>()?;
            let end = end.trim().parse::<u16>()?;
            if start > end {
                bail!("Invalid port range: {}", part);
            }
            for p in start..=end {
                ports.push(p);
            }
        } else {
            ports.push(part.trim().parse::<u16>()?);
        }
    }
    Ok(ports)
}

pub fn parse_targets(target: &str, ports: &str) -> Result<Vec<(String, u16)>> {
    let ip = IpAddr::from_str(target)?;
    let ports_vec = parse_ports(ports)?;
    let mut targets = Vec::new();
    for port in ports_vec {
        targets.push((ip.to_string(), port));
    }
    Ok(targets)
}

pub fn is_private_ip(ip: &IpAddr) -> bool {
    match ip {
        IpAddr::V4(addr) => addr.is_private(),
        IpAddr::V6(_) => false,
    }
}
