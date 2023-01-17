use marine_rs_sdk::marine;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[marine]
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct LFMetadata {
    pub status: String,
    pub power: i32,
}

#[marine]
#[derive(Debug, Serialize, Deserialize)]
pub struct Mint {
    id: String,
    to: String,
    block_no: String,
}

#[derive(Display, Debug, Clone, Copy)]
pub enum FishStatus {
    #[strum(serialize = "normal")]
    NORMAL,

    #[strum(serialize = "decay")]
    DECAY,
}
