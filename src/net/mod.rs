use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json;
use rand;

use crate::{ Player, GameId };

mod client;
mod server;
mod encde;

#[derive(Debug, Serialize, Deserialize)]
enum Message {
    ConnReq(Player),
    RecnReq(Player, GameId),
    StayAlv,
    SyncMsg,
    Ask,
    Err,
}

struct Package {
    content: Vec<u8>,
    len: usize,
    hash: String,
}



impl Package {
    async fn create(msg: Message) -> Result<Self> {
        let mut content = serde_json::to_string(&msg);

        // Ok(Self {

        // })
        todo!()
    }
}

impl Message {

}
