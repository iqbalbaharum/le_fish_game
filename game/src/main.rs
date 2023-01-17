#![allow(improper_ctypes)]

mod metadata;

use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::MountedBinaryResult;
use marine_rs_sdk::WasmLoggerBuilder;
use metadata::FishStatus;
use metadata::LFMetadata;
use metadata::Mint;
use rand::Rng;
use serde_json::json;

module_manifest!();

// configuration
const LAST_MINT_BLOCK: &str = "last_mint_block";
const LAST_FEED_BLOCK: &str = "last_feed_block";
const LAST_BATTLE_BLOCK: &str = "last_battle_block";

pub fn main() {
    WasmLoggerBuilder::new().build().unwrap();
}

#[marine]
pub fn get_latest_mint(uri: String) -> Vec<Mint> {
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
        last_block
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
    // let curl_json = serde_json::from_str(curl_string.as_str()).unwrap();

    let mints: Vec<Mint> = Vec::new();

    // println!("{:?}",curl_json);

    // for(key, value) in curl_json().unwrap().iter() {

    // }

    mints
}

#[marine]
pub fn mint() -> String {
    let metadata = LFMetadata {
        status: FishStatus::NORMAL.to_string(),
        power: rand::thread_rng().gen_range(1..10),
    };

    log::info!("{:?}", metadata);

    serde_json::to_string(&metadata).unwrap()
}

#[marine]
pub fn battle() {}

#[marine]
pub fn feed() {}

#[marine]
pub fn decay() {}

#[marine]
#[link(wasm_import_module = "host")]
extern "C" {
    fn curl(cmd: Vec<String>) -> MountedBinaryResult;
}

#[marine]
#[link(wasm_import_module = "config_file")]
extern "C" {
    // fn add(key: String, value: String) -> LeFishResult;
    fn get_value_by_key(key: String) -> String;
}
