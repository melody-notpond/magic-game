use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use crate::*;
use client::*;
use super::server::ADDR as SERVER_ADDR;
use super::protocol::PROTOCOL_ID;

pub const ADDR: SocketAddr =
    SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 6942);

pub fn client_plugin(
    shared: SharedConfig,
) -> ClientPlugins {
    let io = IoConfig::from_transport(ClientTransport::UdpSocket(ADDR));
    let config = ClientConfig {
        shared,
        net: NetConfig::Netcode {
            // TODO: get this somehow
            auth: Authentication::Manual {
                server_addr: SERVER_ADDR,
                client_id: 0,
                private_key: Key::default(),
                protocol_id: PROTOCOL_ID,
            }, // TODO
            config: NetcodeConfig::default(),
            io,
        },
        ..default()
    };

    ClientPlugins::new(config)
}