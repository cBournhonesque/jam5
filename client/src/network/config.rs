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
            certificate_digest: "d9:b3:06:2d:31:b1:21:6d:c9:8b:24:e6:9f:12:59:23:4d:e2:35:84:4e:b0:cf:2e:ac:4e:b6:ea:ce:56:03:3b".to_string().replace(":", ""),
        },
        Transports::WebSocket => ClientTransport::WebSocketClient { server_addr },
    };

    let mut io = IoConfig::from_transport(transport_config);
    if cfg!(feature = "dev") {
        let link_conditioner = LinkConditionerConfig {
            incoming_latency: Duration::from_millis(40),
            incoming_jitter: Duration::from_millis(10),
            incoming_loss: 0.05,
        };
        io = io.with_conditioner(link_conditioner);
    }
    let config = ClientConfig {
        shared: shared_config(),
        sync: SyncConfig {
            speedup_factor: 1.02,
            ..default()
        },
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
