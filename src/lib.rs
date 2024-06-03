mod net;
mod client;
mod server;

use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct GameId();

// цифорки
#[derive(Debug, Serialize, Deserialize)]
pub struct Nums {
    pub nums: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    msg: String,
    error_kind: ErrorKind,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ErrorKind {
    CorrData,
    WrongNum,
    Disconnected(usize),
    OutOfBounds,
}

impl Error {
    fn new(msg: String, error_kind: ErrorKind) -> Self {
        Self { msg, error_kind }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.msg
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} on {:?}", self.msg, self.error_kind)
    }
}

impl Nums {
    async fn new(nums: Vec<u8>) -> Self {
        let mut v = Vec::with_capacity(12);
        v = nums;
        Self { nums: v }
    }
}
