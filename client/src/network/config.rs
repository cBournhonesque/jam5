use std::net::{Ipv4Addr, SocketAddr};
use std::time::Duration;

use bevy::prelude::default;
use lightyear::prelude::client::*;
use lightyear::prelude::*;

use shared::network::config::{shared_config, Transports, KEY, PROTOCOL_ID};

pub(crate) fn build_lightyear_client(
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
            certificate_digest: "47:44:75:38:88:8b:4c:3f:5f:f7:09:75:dc:58:5e:7f:29:0b:1f:79:64:6e:c8:a9:94:b2:93:56:8f:96:b4:ea".to_string().replace(":", ""),
        },
        Transports::WebSocket => ClientTransport::WebSocketClient { server_addr },
    };

    let mut io = IoConfig::from_transport(transport_config);
    if cfg!(feature = "dev") {
        let link_conditioner = LinkConditionerConfig {
            incoming_latency: Duration::from_millis(40),
            incoming_jitter: Duration::from_millis(4),
            incoming_loss: 0.01,
        };
        io = io.with_conditioner(link_conditioner);
    }
    let config = ClientConfig {
        shared: shared_config(),
        net: NetConfig::Netcode {
            auth,
            config: NetcodeConfig::default(),
            io,
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
