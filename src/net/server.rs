use std::{net::SocketAddr, thread::panicking};

// import a paths from a root of submodule
use super::{Package, messages::Message, Kyber};
use tokio::{ sync::mpsc, net, task };
use log::{info, warn, error, debug};

use anyhow::Result;
use ahash::RandomState;
use rand::rngs::ThreadRng;
use pqc_kyber_kyberslash::*;
use tokio;
use crate::{Error, ErrorKind};

pub struct Server {
    udp_socket: net::UdpSocket,
    crypto: EncDe,
    slots: usize,
    rx_from_threads: mpsc::Receiver<Message>,
    tx_to_balancer: mpsc::Sender<Message>,
}

pub(crate) struct EncDe {
    hasher: RandomState,
    client_keys: Vec<Vec<u8>>,
    shared_secrets: Vec<Vec<u8>>,
    server_keys: Keypair,
    rng: rand::rngs::ThreadRng,
}

struct Thread {
    tx: mpsc::Sender<Message>,
    thread: task::JoinHandle<Result<()>>,
}

impl Server {
    pub fn init(rx_from_threads: mpsc::Receiver<Message>, tx_to_balancer: mpsc::Sender<Message>) -> Result<Self> {
        use std::net as snet;
        let localhost = snet::SocketAddrV4::new(snet::Ipv4Addr::LOCALHOST, 4760);
        let addr = snet::UdpSocket::bind(localhost)?;
        let r = Ok(Server {
            udp_socket: net::UdpSocket::from_std(addr)?,
            crypto: EncDe::init()?,
            slots: 4,
            rx_from_threads,
            tx_to_balancer,
        });
        info!("Local server created");
        r
    }

    async fn send_to(
        &mut self,
        msg: Message,
        target: SocketAddr,
        public_key: &[u8],
    ) -> Result<usize> {
        let mut bytes = match msg {
            Message::SyncMsg(d) => serde_json::to_vec(&d)?,
            Message::StayAlv => serde_json::to_vec("Stay alive!")?,
            Message::Err(e) => serde_json::to_vec(&e)?,
            _ => unreachable!(),
        };
        let (len, hash) = self.crypto.encode(&mut bytes, public_key).await?;
        let package = serde_json::to_vec(&Package {
            msg: bytes,
            len,
            hash,
        })?;

        Ok(self.udp_socket.send_to(&package, target).await?)
    }

    async fn unsafe_send_to(&mut self, msg: Message, target: SocketAddr) -> Result<usize> {
        let bytes = match msg {
            Message::ConnReq(s) => serde_json::to_vec(&s)?,
            _ => unreachable!(),
        };

        let len = bytes.len();
        let hash = self.crypto.hasher.hash_one(&bytes);
        let package = serde_json::to_vec(&Package {
            msg: bytes,
            len,
            hash,
        })?;

        Ok(self.udp_socket.send_to(&package, target).await?)
    }

    async fn recv_loop(&mut self) -> Result<()> {
        let mut buf = Vec::new();
        // let mut from;
        loop {
            let mut package: Package = serde_json::from_slice(&buf)?;
            let encrypted = package.is_encrypted().await;
            let message: Message;

            if encrypted { self.crypto.decode(&mut package.msg, &self.crypto.server_keys).await?; }

            message = serde_json::from_slice(&self.integrity_check(package).await?)?;

            if let Ok(_) = self.udp_socket.recv(&mut buf).await {

            } else {
                break
            }
        }
        Ok(())
    }

    pub async fn ake(&mut self) -> Result<()> {
        let mut bob = Ake::new();

        // let client_init = alice.client_init(&self.crypto.server_key, &mut self.crypto.rng)?;

        // self.unsafe_send(Message::ConnReq(Some(client_init.to_vec()))).await?;

        // alice.client_confirm(server_send, &self.crypto.keys.secret);

        Ok(())
    }

    async fn integrity_check(&self, package: Package) -> Result<Vec<u8>> {
        if package.len != package.msg.len()
            || package.hash != self.crypto.hasher.hash_one(&package.msg)
        { return Err(Error::new("Received corrupted data".to_string(),
                ErrorKind::CorrData).into())}
        Ok(package.msg)
    }

}

impl Thread {
    async fn new() -> Result<Self> {
        let (tx, rx): (_, mpsc::Receiver<Message>) = mpsc::channel(20);
        let thread = task::spawn(async move {
            rx;
            Ok(())
        });
        Ok(Self { tx, thread })
    }
}

impl EncDe {
    pub fn init() -> Result<Self> {
        let hasher: RandomState = RandomState::new();
        let mut rng = rand::thread_rng();

        let client_keys: Vec<Vec<u8>> = Vec::with_capacity(4);
        let shared_secrets: Vec<Vec<u8>> = Vec::with_capacity(4);
        let server_keys = keypair(&mut rng)?;

        Ok(Self {
            hasher,
            client_keys,
            shared_secrets,
            server_keys,
            rng,
        })
    }
}

impl Kyber for EncDe {
    async fn prelude_enc(&mut self) -> Result<(&RandomState, &mut ThreadRng)> {
        Ok((&self.hasher, &mut self.rng))
    }

    async fn prelude_dec(&self) -> Result<&RandomState> {
        Ok(&self.hasher)
    }

    async fn get_public(&self, id: usize) -> Result<&[u8]> {
        if self.client_keys.len() - 1 < id {
            return Err(Error::new("Out of bounds!".to_string(), ErrorKind::OutOfBounds).into());
        }
        Ok(&self.client_keys[id])
    }

    async fn get_private(&self) -> Result<&[u8]> {
        Ok(&self.server_keys.secret)
    }
}
