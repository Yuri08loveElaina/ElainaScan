use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use anyhow::Result;
use std::net::TcpStream;
use std::time::Duration;

pub fn run_finger_nmap(target: &str, ports: &str) -> Result<()> {
    let mut cmd = Command::new("nmap");
    cmd.arg("-O").arg("-sV").arg("-p").arg(ports).arg(target);
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let mut child = cmd.spawn()?;
    let stdout = child.stdout.take().expect("Failed to capture stdout");

    let reader = BufReader::new(stdout);
    for line in reader.lines() {
        let line = line?;
        if line.contains("open") {
            println!("\x1b[92m{}\x1b[0m", line);
        } else if line.contains("OS details") || line.contains("Running") {
            println!("\x1b[93m{}\x1b[0m", line);
        } else {
            println!("{}", line);
        }
    }

    let status = child.wait()?;
    if !status.success() {
        eprintln!("Nmap exited with non-zero status: {:?}", status);
    }

    Ok(())
}

pub fn run_finger_stealth(target: &str, ports: &str, timeout: u64) -> Result<()> {
    use pnet::packet::icmp::echo_request::{MutableEchoRequestPacket};
    use pnet::packet::icmp::IcmpTypes;
    use pnet::packet::ip::IpNextHeaderProtocols;
    use pnet::transport::{transport_channel, icmp_packet_iter, TransportChannelType::Layer4, TransportProtocol::Ipv4};
    use pnet::transport::icmp::IcmpTransportChannelIterator;
    use std::net::Ipv4Addr;

    println!("[*] Stealth OS Fingerprinting using TTL + Banner on {}", target);

    let port_list: Vec<&str> = ports.split(',').collect();
    for port in port_list {
        match TcpStream::connect_timeout(
            &format!("{}:{}", target, port).parse()?,
            Duration::from_millis(timeout)
        ) {
            Ok(mut stream) => {
                stream.set_read_timeout(Some(Duration::from_secs(3)))?;
                let mut buffer = [0u8; 1024];
                match stream.read(&mut buffer) {
                    Ok(n) => {
                        let banner = String::from_utf8_lossy(&buffer[..n]);
                        println!("\x1b[92m[{}:{}] Open - Banner: {}\x1b[0m", target, port, banner.trim());
                    },
                    Err(_) => {
                        println!("\x1b[92m[{}:{}] Open - No banner returned\x1b[0m", target, port);
                    }
                }
            },
            Err(_) => {
                println!("[{}:{}] Closed", target, port);
            }
        }
    }

    println!("[*] TTL-based OS Fingerprint: (manual TTL capture pending in advanced build)");
    println!("  - TTL 64: Linux | 128: Windows | 255: Cisco/BSD (quan sát bằng Wireshark/tcpdump)");

    Ok(())
}
