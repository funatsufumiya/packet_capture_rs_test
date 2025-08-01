// https://qiita.com/m10i/items/f8d3db359f150aafc83b

extern crate netdev;
extern crate pnet;
extern crate pnet_datalink;

use std::{net::IpAddr, thread};

use pnet::packet::{
    ethernet::{EtherTypes, EthernetPacket},
    ip::{IpNextHeaderProtocol, IpNextHeaderProtocols},
    ipv4::Ipv4Packet,
    tcp, Packet,
};
use pnet_datalink::{Channel::Ethernet, DataLinkReceiver, DataLinkSender, NetworkInterface};

pub fn ignition() {
    #[cfg(target_os = "windows")]
    let is_windows = true;
    #[cfg(not(target_os = "windows"))]
    let is_windows = false;

    let mut _best_device_name: String = "\\Device\\NPF_".to_string();

    match netdev::get_default_interface() {
        Ok(_best_interface) => {
            println!("Best Interface Name: {}", _best_interface.name);
            if is_windows {
                _best_device_name += &_best_interface.name;
            } else {
                _best_device_name = _best_interface.name;
            }
            println!(
                "Friendly Name: {:?}",
                _best_interface.friendly_name.unwrap()
            );
        }
        Err(ex) => {
            panic!("Error while find the best interface. {}", ex);
        }
    }

    // すべてのネットワークインターフェースを取得
    let _all_interfaces: Vec<NetworkInterface> = pnet_datalink::interfaces();

    // print all interfaces
    // _all_interfaces.iter().for_each(|iface| println!("Interface: {:?} (is_loopback: {:?})", iface, iface.is_loopback()));

    let _default_interface: Option<NetworkInterface> = _all_interfaces
        .iter()
        .find(|iface| iface.is_loopback() == false && iface.name == _best_device_name)
        .cloned();

    // print default interface
    println!("Default Interface: {:?}", _default_interface);

    if let Some(interface) = _default_interface {
        println!("Found default interface with [{}].", interface.name);

        let builder = thread::Builder::new().name("ro-indexer-packet-caputure".into());

        let handle = builder
            .spawn(move || {
                let (_, mut rx): (Box<dyn DataLinkSender>, Box<dyn DataLinkReceiver>) =
                    match pnet_datalink::channel(&interface, Default::default()) {
                        Ok(Ethernet(tx, rx)) => (tx, rx),
                        Ok(_) => panic!("Unhandled channel type"),
                        Err(ex) => panic!("Unable to create channel: {}", ex),
                    };
                loop {
                    match rx.next() {
                        Ok(_packet) => match EthernetPacket::new(_packet) {
                            Some(frame) => {
                                handle_ethernet_frame(&frame);
                            }
                            _ => {}
                        },
                        Err(ex) => panic!("Unable to recive frame: {}", ex),
                    }
                }
            })
            .unwrap();

        match handle.join() {
            Ok(_) => {
                println!("")
            }
            Err(ex) => {
                println!("Error: {:?}", ex)
            }
        }
    } else {
        panic!("Error while finding the default interface.");
    }
}

fn handle_ethernet_frame(ethernet: &EthernetPacket) {
    match ethernet.get_ethertype() {
        EtherTypes::Ipv4 => handle_ipv4_packet(&ethernet),
        _ => (),
    }
}

fn handle_ipv4_packet(ethernet: &EthernetPacket) {
    let ip_packet: Option<Ipv4Packet> = Ipv4Packet::new(ethernet.payload());

    match ip_packet {
        Some(ip_packet) => {
            handle_transport_protocol(
                IpAddr::V4(ip_packet.get_source()),
                IpAddr::V4(ip_packet.get_destination()),
                ip_packet.get_next_level_protocol(),
                ip_packet.payload(),
            );
        }
        _ => (),
    }
}

fn handle_transport_protocol(
    source: IpAddr,
    destination: IpAddr,
    protocol: IpNextHeaderProtocol,
    packet: &[u8],
) {
    match protocol {
        IpNextHeaderProtocols::Tcp => {
            handle_tcp_segment(source, destination, packet);
        }
        _ => (),
    }
}

fn handle_tcp_segment(source: IpAddr, destination: IpAddr, packet: &[u8]) {
    let segment: Option<tcp::TcpPacket> = tcp::TcpPacket::new(packet);
    match segment {
        Some(segment) => {
            println!(
                "{}:{} -> {}:{}",
                source,
                segment.get_source(),
                destination,
                segment.get_destination()
            );
            //ここでTCPパケットの送信元IPv4アドレス&ポート + 宛先IPv4アドレス&ポートが見える
        }
        _ => (),
    }
}
