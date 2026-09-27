use etherparse::SlicedPacket;
use tun_rs::{AsyncDevice, DeviceBuilder};
use etherparse::NetSlice;
use std::error::Error;

// configuration for the VPN client network interface
struct VpnConfig {
    net1: u8,
    net2: u8,
    net3: u8,
    net4: u8,
    subnet_mask: [u8; 4],
    device_name: &'static str,
}

impl VpnConfig {
    fn new(net1: u8, net2: u8, net3: u8, net4: u8) -> Self {
        Self {
            net1,
            net2,
            net3,
            net4,
            subnet_mask: [255, 255, 255, 0],
            device_name: "LintVPN-Tun",
        }
    }

    fn get_ipv4_addr(&self) -> std::net::Ipv4Addr {
        std::net::Ipv4Addr::new(self.net1, self.net2, self.net3, self.net4)
    }

    fn get_subnet_mask(&self) -> std::net::Ipv4Addr {
        std::net::Ipv4Addr::new(
            self.subnet_mask[0],
            self.subnet_mask[1],
            self.subnet_mask[2],
            self.subnet_mask[3],
        )
    }

    fn print_ip(&self) {
        println!(
            "[VPN CLIENT] SUCCESS STARTED ON IP: {}.{}.{}.{}",
            self.net1, self.net2, self.net3, self.net4
        );
    }
}

// builds async TUN device with the provided configuration
fn build_device(config: &VpnConfig) -> Result<AsyncDevice, Box<dyn Error>> {
    let mut builder = DeviceBuilder::new();

    builder = builder.ipv4(
        config.get_ipv4_addr(),
        config.get_subnet_mask(),
        None,
    );

    #[cfg(target_os = "windows")]
    {
        builder = builder.name(config.device_name);
    }

    Ok(builder.build_async()?)
}

// processes a single packet and prints information about it
fn process_packet(packet_bytes: &[u8]) -> Result<(), Box<dyn Error>> {
    let packet = SlicedPacket::from_ip(packet_bytes)?;

    if let Some(NetSlice::Ipv4(ipv4_header)) = packet.net {
        let header = ipv4_header.header();
        let src = header.source_addr();
        let dst = header.destination_addr();
        let proto = header.protocol();

        println!(
            "[VPN CLIENT] PACKET INTERCEPTED {} -> {} | PROTOCOL: {:?}",
            src, dst, proto
        );
    }

    Ok(())
}

// main packet reading loop
async fn packet_loop(dev: AsyncDevice) -> Result<(), Box<dyn Error>> {
    let mut buf = vec![0u8; 1500];

    loop {
        let n = dev.recv(&mut buf).await?;
        let packet_bytes = &buf[..n];

        if let Err(e) = process_packet(packet_bytes) {
            eprintln!("[VPN CLIENT] ERROR processing packet: {}", e);
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("=== [VPN CLIENT]: STARTED NET INTERFACE ===");

    // initialize configuration (change these values for different virtual IPs)
    let config = VpnConfig::new(10, 8, 0, 2);

    // build async device
    let dev = build_device(&config)?;
    config.print_ip();

    // start packet processing loop
    packet_loop(dev).await
}
