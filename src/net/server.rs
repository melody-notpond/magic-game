use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use crate::*;
use server::*;
use super::protocol::PROTOCOL_ID;

pub const ADDR: SocketAddr =
    SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 42069);

pub fn server_plugin(
    shared: SharedConfig,
) -> ServerPlugins {
    let io = IoConfig::from_transport(ServerTransport::UdpSocket(ADDR));
    ServerPlugins {
        config: ServerConfig {
            shared,
            net: vec![
                NetConfig::Netcode {
                    config: NetcodeConfig::default()
                        .with_protocol_id(PROTOCOL_ID)
                        // TODO: how private key
                        .with_key(Key::default()),
                    io,
                },
            ],
            ..default()
        }
    }
}
