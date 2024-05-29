use crate::{
    Player,
    net::*,
};


use ahash::AHashMap as HashMap;
use anyhow::Result;
use tokio::{self, net as tokionet, time, fs};
use log::{info, warn, error, debug};
use serde_json;


// фулл? инфа про всех игроков
struct Host {
    udp_socket: tokionet::UdpSocket,
    players: HashMap<u8, Player>,
}

impl Host {
    fn init() -> Result<Self> {
        use std::net as snet;
        let localhost = snet::SocketAddrV4::new(snet::Ipv4Addr::LOCALHOST, 4760);
        let addr = snet::UdpSocket::bind(localhost)?;
        let r = Ok(Host {
            udp_socket: tokionet::UdpSocket::from_std(addr)?,
            players: HashMap::with_capacity(4),
        });
        info!("Host created");
        r
    }

    async fn waiting_for_players(&mut self) -> Result<()> {
        let mut buf = Vec::new();
        while let Ok(bytes) = self.udp_socket.recv(&mut buf).await {
            // let package:  = serde_json::from_slice(&buf)?;

            buf.clear();
        }
        Ok(())
    }
}
