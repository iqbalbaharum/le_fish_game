#![allow(improper_ctypes)]

mod metadata;

use chrono::{TimeZone, Utc};
use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::MountedBinaryResult;
use marine_rs_sdk::WasmLoggerBuilder;
use metadata::FishStatus;
use metadata::LFMetadata;
use metadata::{Mint, PvPBattleOutcome};
use rand::Rng;
use serde_json::json;

use std::time::{SystemTime, UNIX_EPOCH};
use types::ConfigResult;

module_manifest!();

// configuration
const LAST_MINT_BLOCK: &str = "last_mint_block";
const LAST_FEED_BLOCK: &str = "last_feed_block";
const MARGIN_BATTLE_PERCENTAGE: f32 = 0.2;

pub fn main() {
    WasmLoggerBuilder::new().build().unwrap();
}

#[marine]
pub fn get_latest_mint_event(uri: String) -> Vec<Mint> {
    // get lastest block
    let mut last_block = 0;

    let config_last_block = get_value_by_key(LAST_MINT_BLOCK.to_string());

    if !config_last_block.is_empty() {
        last_block = config_last_block.parse().unwrap()
    }

    log::info!("Last process block: {}", last_block);

    let query = format!(
        r#"query {{
        mintBurnMints (
            orderBy: blockNo,
            orderDirection: asc,
            where: {{
                blockNo_gt: {}
            }},
        ) {{
        id
        to
        blockNo
    }}
    }}"#,
        last_block as u64
    );

    let query_json = json!({ "query": query });
    let json_str = serde_json::to_string(&query_json).unwrap();

    let args = vec![
        String::from("-X"),
        String::from("POST"),
        String::from("-H"),
        String::from("Content-Type: application/json"),
        String::from("-d"),
        json_str,
        uri,
    ];

    let response = curl(args);
    let curl_string = String::from_utf8(response.stdout).unwrap();
    println!("{:?}", curl_string);

    let mints: Vec<Mint> = Vec::new();

    mints
}

#[marine]
pub fn get_initial_fish_power() -> i32 {
    rand::thread_rng().gen_range(1..10)
}

#[marine]
pub fn stringify_i32(number: i32) -> String {
    number.to_string()
}

#[marine]
pub fn mint(power: i32) -> String {
    let metadata = LFMetadata {
        status: FishStatus::NORMAL.to_string(),
        power,
        timestamp: get_server_timestamp(),
        total_feed: 0,
        total_battle: 0,
        total_decay: 0,
    };

    log::info!("{:?}", metadata);

    serde_json::to_string(&metadata).unwrap()
}

#[marine]
pub fn battle(fish1_metadata: String, fish2_metadata: String) -> PvPBattleOutcome {
    let mut fish1_json: LFMetadata = serde_json::from_str(fish1_metadata.as_ref()).unwrap();
    let mut fish2_json: LFMetadata = serde_json::from_str(fish2_metadata.as_ref()).unwrap();

    let fish_1_bp = fish1_json.power;
    let fish_2_bp = fish2_json.power;

    if fish1_json.power > fish2_json.power {
        fish1_json.power = fish1_json.power
            + ((fish_2_bp.clone() as f32 * MARGIN_BATTLE_PERCENTAGE).ceil() as i32);
        fish2_json.power = fish2_json.power
            - ((fish_1_bp.clone() as f32 * MARGIN_BATTLE_PERCENTAGE).ceil() as i32);

        if fish2_json.power <= 0 {
            fish2_json.power = 1
        }
    } else {
        fish2_json.power = fish2_json.power
            + ((fish_1_bp.clone() as f32 * MARGIN_BATTLE_PERCENTAGE).ceil() as i32);
        fish1_json.power = fish1_json.power
            - ((fish_2_bp.clone() as f32 * MARGIN_BATTLE_PERCENTAGE).ceil() as i32);

        if fish1_json.power <= 0 {
            fish1_json.power = 1
        }
    }

    fish1_json.total_battle = fish1_json.total_battle + 1;
    fish1_json.timestamp = get_server_timestamp();
    fish2_json.timestamp = get_server_timestamp();

    let fish1_new_json = serde_json::to_string(&fish1_json).unwrap();

    let fish2_new_json = serde_json::to_string(&fish2_json).unwrap();

    log::info!("fish 1 {:?}", fish1_new_json);
    log::info!("fish 2 {:?}", fish2_new_json);

    PvPBattleOutcome {
        fish1: fish1_new_json,
        fish2: fish2_new_json,
    }
}

#[marine]
pub fn feed(source_metadata: String, food_metadata: String) -> String {
    let mut source_json: LFMetadata =
        serde_json::from_str(source_metadata.as_ref()).unwrap_or(LFMetadata {
            status: FishStatus::NORMAL.to_string(),
            power: 0,
            timestamp: get_server_timestamp(),
            total_feed: 0,
            total_battle: 0,
            total_decay: 0,
        });

    let food_json: LFMetadata =
        serde_json::from_str(food_metadata.as_ref()).unwrap_or(LFMetadata {
            status: FishStatus::NORMAL.to_string(),
            power: 0,
            timestamp: get_server_timestamp(),
            total_feed: 0,
            total_battle: 0,
            total_decay: 0,
        });

    source_json.power = source_json.power + food_json.power;
    source_json.total_feed = source_json.total_feed + 1;

    let source_new_json = serde_json::to_string(&source_json).unwrap();

    log::info!("{:?}", source_new_json);

    source_new_json
}

#[marine]
pub fn decay(fish_metadata: String) -> String {
    let mut fish_json: LFMetadata = serde_json::from_str(fish_metadata.as_ref()).unwrap();

    if fish_json.status == FishStatus::DECAY.to_string() {
        if (get_server_timestamp() - fish_json.timestamp) > 86400000 && fish_json.power > 1 {
            fish_json.power = fish_json.power - 1;
            fish_json.timestamp = get_server_timestamp();
        }
    } else {
        if (get_server_timestamp() - fish_json.timestamp) > 172800000 {
            fish_json.status = FishStatus::DECAY.to_string();
            fish_json.timestamp = get_server_timestamp();
            fish_json.total_decay = fish_json.total_decay + 1;
        }
    }

    let fish_new_json = serde_json::to_string(&fish_json).unwrap();

    log::info!("Fish new metadata {:?}", fish_new_json);

    fish_new_json
}

#[marine]
pub fn config_add(key: String, value: String) -> ConfigResult {
    add(key, value)
}

#[marine]
pub fn config_get(key: String) -> String {
    get_value_by_key(key)
}

fn get_server_timestamp() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    since_the_epoch.as_millis() as u64
}

#[marine]
#[link(wasm_import_module = "host")]
extern "C" {
    fn curl(cmd: Vec<String>) -> MountedBinaryResult;
}

#[marine]
#[link(wasm_import_module = "config_file")]
extern "C" {
    fn add(key: String, value: String) -> ConfigResult;
    fn get_value_by_key(key: String) -> String;
}
