use dotenvy::dotenv;
use etherparse::SlicedPacket;
use std::env;
use std::error::Error;
use std::net::{Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tun_rs::{AsyncDevice, DeviceBuilder};

// configuration for the VPN client network interface
struct VpnConfig {
    client_ip: Ipv4Addr,
    subnet_mask: Ipv4Addr,
    device_name: String,
    server_addr: String,
}

impl VpnConfig {
    fn from_env() -> Result<Self, Box<dyn Error>> {
        dotenv().ok();

        let client_ip_str = env::var("VPN_CLIENT_IP").unwrap_or_else(|_| "10.8.0.2".to_string());
        let subnet_mask_str =
            env::var("VPN_SUBNET_MASK").unwrap_or_else(|_| "255.255.255.0".to_string());
        let device_name =
            env::var("VPN_DEVICE_NAME").unwrap_or_else(|_| "LintVPN-Tun".to_string());
        let server_addr =
            env::var("VPN_SERVER_ADDR").expect("VPN_SERVER_ADDR must be set in .env file");

        Ok(Self {
            client_ip: Ipv4Addr::from_str(&client_ip_str)?,
            subnet_mask: Ipv4Addr::from_str(&subnet_mask_str)?,
            device_name,
            server_addr,
        })
    }
}


// builds async TUN device with the provided configuration
fn build_device(config: &VpnConfig) -> Result<AsyncDevice, Box<dyn Error>> {
    let mut builder = DeviceBuilder::new();
    builder = builder.ipv4(config.client_ip, config.subnet_mask, None);

    #[cfg(target_os = "windows")]
    {
        builder = builder.name(&config.device_name);
    }

    Ok(builder.build_async()?)
}

fn log_packet(direction: &str, packet_bytes: &[u8]) {
    if let Ok(packet) = SlicedPacket::from_ip(packet_bytes) {
        if let Some(etherparse::NetSlice::Ipv4(hdr)) = packet.net {
            let header = hdr.header();
            println!(
                "{} {} -> {} | Protocol: {:?}",
                direction,
                header.source_addr(),
                header.destination_addr(),
                header.protocol()
            );
        }
    }
}

/*
TASK 1: read from TUN -> transfer on UDP at VPS
TASk 2: read from UDP -> write to local TUN
 */

fn main() {}
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     println!("=== [VPN CLIENT]: INITIALIZING ===");
//
//     // load configuration
//     let config = VpnConfig::from_env()?;
//     let server_addr: SocketAddr = config.server_addr.parse()?;
//
//     // create TUN-interface
//     let dev = Arc::new(build_device(&config)?);
//     println!("[VPN CLIENT] TUN Interface created: {}", config.client_ip);
//
//     // build local UDP-socket
//     let socket = Arc::new(UdpSocket::bind("0.0.0.0:0").await?);
//     println!(
//         "[VPN CLIENT] Socket bound to local port: {}",
//         socket.local_addr()?
//     );
//     println!("[VPN CLIENT] Target server: {}", server_addr);
// }
