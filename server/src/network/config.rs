use async_compat::Compat;
use bevy::log::info;
use bevy::prelude::default;
use bevy::tasks::IoTaskPool;
use lightyear::prelude::server::*;
use lightyear::prelude::*;
use std::net::{Ipv4Addr, SocketAddr};
use std::time::Duration;

use shared::network::config::{shared_config, Transports, KEY, PROTOCOL_ID};

pub(crate) fn build_lightyear_server(port: u16, transport: Transports) -> ServerPlugins {
    // Step 1: create the io (transport + link conditioner)
    let server_addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), port);
    let transport_config = match transport {
        Transports::Udp => ServerTransport::UdpSocket(server_addr),
        // if using webtransport, we load the certificate keys
        Transports::WebTransport => {
            // this is async because we need to load the certificate from io
            // we need async_compat because wtransport expects a tokio reactor
            let certificate = IoTaskPool::get()
                .scope(|s| {
                    s.spawn(Compat::new(async {
                        server::Identity::load_pemfiles(
                            "../client/assets/certificates/cert.pem",
                            "../client/assets/certificates/key.pem",
                        )
                        .await
                        .unwrap()
                    }));
                })
                .pop()
                .unwrap();
            let digest = certificate.certificate_chain().as_slice()[0].hash();
            info!("Generated self-signed certificate with digest: {}", digest);
            ServerTransport::WebTransportServer {
                server_addr,
                certificate,
            }
        }
        Transports::WebSocket => ServerTransport::WebSocketServer { server_addr },
    };
    let link_conditioner = LinkConditionerConfig {
        incoming_latency: Duration::from_millis(0),
        incoming_jitter: Duration::from_millis(0),
        incoming_loss: 0.0,
    };
    // Step 2: define the server configuration
    let shared_config = shared_config();
    let replication_config = ReplicationConfig {
        send_updates_mode: SendUpdatesMode::SinceLastAck,
        send_interval: shared_config.server_replication_send_interval,
    };
    let config = ServerConfig {
        shared: shared_config,
        net: vec![NetConfig::Netcode {
            config: NetcodeConfig::default()
                .with_protocol_id(PROTOCOL_ID)
                .with_key(KEY),
            io: IoConfig::from_transport(transport_config).with_conditioner(link_conditioner),
        }],
        replication: replication_config,
        ..default()
    };

    // Step 3: create the plugin
    ServerPlugins::new(config)
}
