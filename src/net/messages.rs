use serde::{Deserialize, Serialize};
use crate::Error;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum Message {
    ConnReq(Option<Vec<u8>>),
    RecnReq,
    SyncMsg(SyncMsg),
    StayAlv,
    Ake(Vec<u8>),
    Ack,
    Err(Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum SyncMsg {
    Number(u8),
    Level(Level),
    Star,
    Ping(usize),
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum Level {
    Next(Vec<u8>),
    Win,
    Lose,
}
