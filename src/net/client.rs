use std::net;

// import a paths from a root of submodule
use super::{Package, messages::Message, Kyber};
// use log::{info, warn, error, debug};

use anyhow::Result;
use ahash::RandomState;
use rand::{rngs::ThreadRng, Rng};
use pqc_kyber_kyberslash::*;
use tokio;
// use crate::{ Error, ErrorKind, Nums };

pub struct Client {
    udp_socket: tokio::net::UdpSocket,
    crypto: EncDe,
}

struct EncDe {
    hasher: RandomState,
    server_key: [u8; 1568],
    keys: Keypair,
    rng: rand::rngs::ThreadRng,
}

impl Client {
    async fn init(target: net::SocketAddr) -> Result<Self> {
        let localhost = net::SocketAddrV4::new(net::Ipv4Addr::LOCALHOST, 58600);
        let sock = net::UdpSocket::bind(localhost)?;
        let sock = tokio::net::UdpSocket::from_std(sock)?;
        sock.connect(target).await?;
        Ok(Self {
            udp_socket: sock,
            crypto: EncDe::init()?,
        })
    }

    async fn send(&mut self, msg: Message) -> Result<()> {
        let mut bytes = serde_json::to_vec(&msg)?;

        let server_key = self.crypto.server_key;
        let (len, hash) = self.crypto.encode(&mut bytes, &server_key).await?;
        let package = serde_json::to_vec(&Package {
            msg: bytes,
            len,
            hash,
        })?;
        self.udp_socket.send(&package).await?;

        Ok(())
    }

    async fn unsafe_send(&mut self, msg: Message) -> Result<usize> {
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

        Ok(self.udp_socket.send(&package).await?)
    }

    pub async fn ake(&mut self) -> Result<()> {
        let mut alice = Ake::new();
        let client_init = alice.client_init(&self.crypto.server_key, &mut self.crypto.rng)?;

        self.unsafe_send(Message::ConnReq(Some(client_init.to_vec())))
            .await?;

        // alice.client_confirm(server_send, &self.crypto.keys.secret);

        Ok(())
    }
}

impl EncDe {
    fn init() -> Result<Self> {
        let hasher: RandomState = RandomState::new();
        let mut rng = rand::thread_rng();

        let server_key: [u8; 1568] = [0; 1568];
        let keys = keypair(&mut rng)?;

        Ok(Self {
            hasher,
            server_key,
            keys,
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

    async fn get_public(&self, _: usize) -> Result<&[u8]> {
        Ok(&self.keys.public)
    }

    async fn get_private(&self) -> Result<&[u8]> {
        Ok(&self.keys.secret)
    }
}
