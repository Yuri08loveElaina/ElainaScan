use pnet::transport::{transport_channel, TransportChannelType::Layer3, icmp_packet_iter};
use pnet::packet::icmp::{echo_request, IcmpTypes};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::{MutablePacket, Packet};
use std::net::Ipv4Addr;
use std::time::Duration;
use anyhow::{Result, anyhow};

pub async fn fingerprint_ttl(ip: &str) -> Result<u8> {
    tokio::task::spawn_blocking(move || {
        let protocol = Layer3(IpNextHeaderProtocols::Icmp);
        let (mut tx, mut rx) = transport_channel(1024, protocol)?;
        let mut buf = [0u8; 64];
        let mut packet = echo_request::MutableEchoRequestPacket::new(&mut buf).ok_or_else(|| anyhow!("Failed to create ICMP packet"))?;
        packet.set_icmp_type(IcmpTypes::EchoRequest);
        packet.set_sequence_number(0);
        packet.set_identifier(0);
        let checksum = pnet::util::checksum(packet.packet(), 1);
        packet.set_checksum(checksum);

        let target_ip: Ipv4Addr = ip.parse()?;
        tx.send_to(packet, target_ip.into())?;

        let mut iter = icmp_packet_iter(&mut rx);
        let start = std::time::Instant::now();

        while start.elapsed() < Duration::from_secs(2) {
            if let Ok((packet, addr)) = iter.next() {
                if addr == target_ip {
                    let ttl = packet.get_ttl();
                    return Ok(ttl);
                }
            }
        }
        Err(anyhow!("Timeout or no ICMP response"))
    }).await?
}
