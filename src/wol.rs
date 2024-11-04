use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};

pub trait Wake {
    fn wake(&self) -> ();
}

pub fn wake_on_lan(mac_address: &[u8; 12], port: &u16) -> () {
    let address_str = mac_address
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<String>>()
        .join(":");
    print!(
        "[WoL] Sending magic packet to host: {}, port: {} ->",
        address_str, port
    );

    let packet = create_magic_packet(&mac_address);
    let socket: UdpSocket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind udp socket");
    let broadcast_addr = SocketAddrV4::new(Ipv4Addr::BROADCAST, port.to_owned());
    let _ = socket.send_to(&packet, broadcast_addr);

    print!(
        "[WoL] Magic packet sent to host: {}, port: {} ->",
        address_str, port
    );
}

fn create_magic_packet(mac: &[u8; 12]) -> Vec<u8> {
    let mut packet = Vec::with_capacity(102);

    packet.extend([0xFF; 6]);

    for _ in 0..16 {
        packet.extend_from_slice(mac);
    }

    packet
}
