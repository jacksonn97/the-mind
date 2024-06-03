use ahash::RandomState;
// use log::{info, warn, error, debug};
use kyber_pke::{encrypt, decrypt};
use pqc_kyber_kyberslash::Keypair;
use rand::{rngs::ThreadRng, Rng};
use anyhow::Result;
use serde::{Deserialize, Serialize};

const PREFIX: &[u8] = "---ENCRYPTED-MESSAGE---".as_bytes();
const PREFIX_LEN: usize = PREFIX.len();

pub mod client;
pub mod messages;
pub mod server;

#[derive(Debug, Serialize, Deserialize)]
struct Package {
    pub msg: Vec<u8>,
    pub len: usize,
    pub hash: u64,
}

impl Package {
    async fn is_encrypted(&self) -> bool {
        self.msg.starts_with(PREFIX)
    }
}

trait Kyber {
    async fn prelude_enc(&mut self) -> Result<(&RandomState, &mut ThreadRng)>;
    async fn prelude_dec(&self) -> Result<(&RandomState)>;
    async fn get_public(&self, id: usize) -> Result<&[u8]>;
    async fn get_private(&self) -> Result<&[u8]>;

    async fn encode(&mut self, s: &mut Vec<u8>, public_key: &[u8]) -> Result<(usize, u64)> {
        let (hasher, mut rng) = self.prelude_enc().await?;
        let hash = hasher.hash_one(&s);
        let len = s.len();

        let mut nonce: [u8; 32] = [0; 32];

        for i in 0..32 {
            nonce[i] = rng.gen();
        }

        // *s = encrypt(&open_key, &s, &nonce)?;
        if let Ok(res) = encrypt(&public_key, &s, &nonce) {
            *s = [ PREFIX, &res ].concat()
        }



        Ok((len, hash))
    }

    async fn decode(&self, s: &mut Vec<u8>, keys: &Keypair) -> Result<()> {
        let hasher = self.prelude_dec().await?;

        // *s = decrypt(&s, &secret_key)?;
        if let Ok(res) = decrypt(&s[PREFIX_LEN..], &keys.secret) {
            *s = res
        }

        let hash = hasher.hash_one(&s);
        let len = s.len();
        Ok(())
    }
}

#[tokio::test]
async fn prefix_bounds() {
    let s: Vec<u8> = "some message".as_bytes().into();

    let new = [ PREFIX, &s ].concat();
    assert_eq!(s, new[PREFIX_LEN..]);


    let package_true = Package { len: 0, hash: 123, msg: new };
    let package_false = Package { len: 0, hash: 123, msg: s };

    assert!(package_true.is_encrypted().await);
    assert!(!package_false.is_encrypted().await)
}

#[tokio::test]
async fn encde() {
    use pqc_kyber_kyberslash::keypair;
    let msg = "some silly message".as_bytes();
    let mut rng = rand::thread_rng();

    let keys_a = keypair(&mut rng).unwrap();

    let mut nonce: [u8; 32] = [0; 32];

    for i in 0..32 {
        nonce[i] = rng.gen();
    }

    let enc = encrypt(&keys_a.public, &msg, &nonce).unwrap();
    let new = decrypt(&keys_a.secret, enc).unwrap();
    assert_eq!(new, msg);
}
