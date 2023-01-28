use marine_rs_sdk::marine;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[marine]
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct LFMetadata {
    pub status: String,
    pub power: i32,
    pub timestamp: u64,
    pub total_battle: u64,
    pub total_feed: u64,
    pub total_decay: u64,
}

#[marine]
#[derive(Debug, Serialize, Deserialize)]
pub struct Mint {
    id: String,
    to: String,
    block_no: String,
}

#[marine]
#[derive(Debug, Serialize, Deserialize)]
pub struct PvPBattleOutcome {
    pub fish1: String,
    pub fish2: String,
}

#[derive(Display, Debug, Clone, Copy)]
pub enum FishStatus {
    #[strum(serialize = "normal")]
    NORMAL,

    #[strum(serialize = "decay")]
    DECAY,
}
