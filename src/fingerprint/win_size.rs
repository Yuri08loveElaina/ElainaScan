use tokio::net::TcpStream;
use std::time::Duration;
use anyhow::{Result, anyhow};
use pnet::packet::tcp::{TcpPacket};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::transport::{transport_channel, TransportChannelType::Layer4, tcp_packet_iter};
use pnet::transport::TransportProtocol::Ipv4;
use pnet::packet::{Packet, MutablePacket};
use std::net::{Ipv4Addr, SocketAddrV4};

pub async fn fingerprint_win_size(ip: &str, port: u16) -> Result<u16> {
    tokio::task::spawn_blocking(move || {
        let protocol = Layer4(Ipv4(IpNextHeaderProtocols::Tcp));
        let (mut tx, mut rx) = transport_channel(4096, protocol)?;
        let source_ip = "0.0.0.0".parse::<Ipv4Addr>()?;
        let source_port = 40000;
        let target_ip: Ipv4Addr = ip.parse()?;
        let target = SocketAddrV4::new(target_ip, port);

        let mut tcp_buffer = [0u8; 40];
        let mut tcp_packet = TcpPacket::new(&mut tcp_buffer).ok_or_else(|| anyhow!("Failed to create TCP packet"))?;
        tcp_packet.set_source(source_port);
        tcp_packet.set_destination(port);
        tcp_packet.set_sequence(0);
        tcp_packet.set_acknowledgement(0);
        tcp_packet.set_data_offset(5);
        tcp_packet.set_flags(0x02); // SYN
        tcp_packet.set_window(64240);
        let checksum = pnet::packet::tcp::ipv4_checksum(&tcp_packet, &source_ip, &target_ip);
        tcp_packet.set_checksum(checksum);

        tx.send_to(tcp_packet, target.into())?;

        let mut iter = tcp_packet_iter(&mut rx);
        let start = std::time::Instant::now();

        while start.elapsed() < Duration::from_secs(3) {
            if let Ok((packet, addr)) = iter.next() {
                if addr.ip() == target_ip && packet.get_destination() == source_port {
                    let win_size = packet.get_window();
                    return Ok(win_size);
                }
            }
        }
        Err(anyhow!("Timeout or no SYN/ACK received"))
    }).
