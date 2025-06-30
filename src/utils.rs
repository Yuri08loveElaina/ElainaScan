pub fn parse_ports(port_range: &str) -> Vec<u16> {
    let mut ports = Vec::new();
    if let Some((start, end)) = port_range.split_once('-') {
        let start: u16 = start.parse().unwrap_or(1);
        let end: u16 = end.parse().unwrap_or(65535);
        for p in start..=end {
            ports.push(p);
        }
    } else if let Ok(single_port) = port_range.parse::<u16>() {
        ports.push(single_port);
    }
    ports
}
