use std::net::{Ipv4Addr, SocketAddr};
use std::time::Duration;

use bevy::prelude::default;
use lightyear::prelude::*;
use lightyear::prelude::client::*;

use shared::network::config::{KEY, PROTOCOL_ID, shared_config, Transports};

pub(crate) fn build_lightyear_plugins(
    client_id: u64,
    client_port: u16,
    server_addr: SocketAddr,
    transport: Transports,
) -> ClientPlugins {
    let auth = Authentication::Manual {
        server_addr,
        client_id,
        private_key: KEY,
        protocol_id: PROTOCOL_ID,
    };
    let client_addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), client_port);
    let transport_config = match transport {
        #[cfg(not(target_family = "wasm"))]
        Transports::Udp => ClientTransport::UdpSocket(client_addr),
        Transports::WebTransport => ClientTransport::WebTransportClient {
            client_addr,
            server_addr,
            #[cfg(target_family = "wasm")]
            certificate_digest,
        },
        Transports::WebSocket => ClientTransport::WebSocketClient { server_addr },
    };
    let link_conditioner = LinkConditionerConfig {
        incoming_latency: Duration::from_millis(40),
        incoming_jitter: Duration::from_millis(4),
        incoming_loss: 0.01,
    };
    let config = ClientConfig {
        shared: shared_config(),
        net: NetConfig::Netcode {
            auth,
            config: NetcodeConfig::default(),
            io: IoConfig::from_transport(transport_config).with_conditioner(link_conditioner),
        },
        interpolation: InterpolationConfig {
            delay: InterpolationDelay::default().with_send_interval_ratio(2.0),
            // do not do linear interpolation per component, instead we provide our own interpolation logic
            // custom_interpolation_logic: true,
        },
        ..default()
    };
    ClientPlugins::new(config)
}