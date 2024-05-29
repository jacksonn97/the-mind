mod net;
mod player_space;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct GameId();

// инфа для клиента т.е. адрес сервера и набор чисел
#[derive(Debug, Serialize, Deserialize)]
struct Player {
    nick: String,
    nums: Nums,
    host: bool,
    // remote_host:
}

// цифорки
#[derive(Debug, Serialize, Deserialize)]
struct Nums {
    nums: Vec<u8>,
}

impl Player {
    async fn send(&mut self) -> Result<()> {
        todo!()
    }
}

impl Nums {
    async fn pop(&mut self) -> u8 {
        todo!()
    }

    async fn set(&mut self, nums: Self) {
        todo!()
    }
}
