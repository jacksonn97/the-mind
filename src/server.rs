use ahash::AHashMap as HashMap;
use rand::{self, rngs::ThreadRng, prelude::SliceRandom};
use crate::{ net::{ server, messages::Message }, Nums };
use anyhow::Result;
use serde::{Deserialize, Serialize};

pub(crate) struct Room {
    stack: Vec<u8>,
    raw: Vec<u8>,
    players: HashMap<usize, Player>,
    stars: usize,
    health: usize,
    level: usize,
    rng: ThreadRng,
}

// инфа для клиента т.е. адрес сервера и набор чисел
#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    nick: String,
    id: usize,
    pub nums: Nums,
    host: bool,
}

impl Room {
    #[inline]
    pub fn new() -> Result<Self> {
        Ok(Self {
            stack: Vec::with_capacity(50),
            raw: (0..=100).collect(),
            players: HashMap::with_capacity(4),
            health: 0,
            stars: 0,
            level: 0,
            rng: rand::thread_rng(),
        })
    }

    async fn lvl_up(&mut self) {
        self.stack.clear();
        self.raw.shuffle(&mut self.rng);
        self.pickups().await;

        for (id, (_, player)) in &mut self.players.iter_mut().enumerate() {
            let mut nums = self
                .raw
                .get(self.level * id..self.level * id + 1)
                .unwrap()
                .to_owned();
            nums.sort();
            nums.reverse();
            player.set_nums(nums).await;
        }
    }

    #[inline]
    async fn pickups(&mut self) {
        if [2, 5, 8].contains(&self.level) {
            self.stars += 1;
            self.stars = self.stars.clamp(0, 3);
        }
        if [3, 6, 9].contains(&self.level) {
            self.health += 1;
            self.health = self.health.clamp(0, 5);
        }
    }


}

impl Player {
    pub async fn new(nick: String, nums: Nums, host: bool) -> Self {
        Self {
            nick,
            id: 0,
            nums,
            host,
        }
    }

    pub async fn pop_num(&mut self) -> Option<u8> {
        self.nums.nums.pop()
    }

    pub async fn set_nums(&mut self, nums: Vec<u8>) {
        self.nums.nums = nums;
    }
}

