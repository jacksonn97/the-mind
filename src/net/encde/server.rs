use ahash::RandomState;
use anyhow::Result;
use pqc_kyber::*;

struct EncDe {
    hasher: RandomState,
    client_keys: Vec<Vec<u8>>,
    shared_secrets: Vec<Vec<u8>>,
    server_keys: Keypair,
    rng: rand::rngs::ThreadRng,
}

impl EncDe {
    fn init() -> Result<Self> {
        let hasher: RandomState = RandomState::new();
        let mut rng = rand::thread_rng();

        let client_keys: Vec<Vec<u8>> = Vec::with_capacity(4);
        let shared_secrets: Vec<Vec<u8>> = Vec::with_capacity(4);
        let server_keys = keypair(&mut rng)?;

        Ok( Self {
            hasher,
            client_keys,
            shared_secrets,
            server_keys,
            rng,
        })
    }

    async fn encode(&mut self, s: &mut String, id: u8) -> Result<(usize, u64)> {
        let hash = self.hasher.hash_one(&s);
        let len = s.len();

        (s, _) = encapsulate(&self.server_keys.secret, &mut self.rng)?;

        Ok((len, hash))
    }
}

