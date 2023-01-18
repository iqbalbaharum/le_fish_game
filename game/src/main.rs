#![allow(improper_ctypes)]


use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::MountedBinaryResult;
use marine_rs_sdk::WasmLoggerBuilder;
use serde_json::json;
use serde_json::{from_str, Value};
use serde::{Deserialize, Serialize};
use rand::Rng;

module_manifest!();

#[derive(Debug, Deserialize)]
pub struct Mint{
    id: String,
    to: String,
    blockNo: String
}

#[derive(Serialize, Deserialize, Debug)]
struct FishStat {
    battle_power: u32,
    stat: String,
}

pub fn main() {
    WasmLoggerBuilder::new().build().unwrap();
}

#[marine]
pub fn process_latest_block(address: String) {
    let last_block = 0;

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
        address,
    ];

    let response = curl(args);
    let curl_string = String::from_utf8(response.stdout).unwrap();
    println!("{:?}",curl_string);
    // let curl_json = serde_json::from_str(curl_string.as_str()).unwrap();

    // println!("{:?}",curl_json);
    
    // for(key, value) in curl_json().unwrap().iter() {
        
    // }
}

#[marine]
pub fn generate_fish(){
    let metadata_json = json!({
        "battle_power": (rand::random::<u32>() % 9) + 1,
        "stat": "normal"
    });

    let metadata_stringify = metadata_json.to_string();

    println!("{:?}", metadata_stringify);
}

#[marine]
pub fn feed_fish(source: String, food:String){
    
    let mut source_json: FishStat = serde_json::from_str(source.as_ref()).unwrap();
    let food_json: FishStat = serde_json::from_str(food.as_ref()).unwrap();

    source_json.battle_power = source_json.battle_power + food_json.battle_power;

    let source_new_json = serde_json::to_string(&source_json).unwrap();

    println!("{:?}", source_new_json);
   
}

#[marine]
pub fn battle_fish(fish1: String, fish2:String){
    
    let mut fish1_json: FishStat = serde_json::from_str(fish1.as_ref()).unwrap();
    let mut fish2_json: FishStat = serde_json::from_str(fish2.as_ref()).unwrap();

    if fish1_json.battle_power > fish2_json.battle_power {
        fish1_json.battle_power = fish1_json.battle_power + (fish2_json.battle_power as f32 * 0.2).floor() as u32;
    }else{
        fish2_json.battle_power = fish2_json.battle_power + (fish1_json.battle_power as f32 * 0.2).floor() as u32;
    }

    let fish1_new_json = serde_json::to_string(&fish1_json).unwrap();

    let fish2_new_json = serde_json::to_string(&fish2_json).unwrap();

    println!("fish 1 {:?}", fish1_new_json);

    println!("fish 2 {:?}", fish2_new_json);
   
}

#[marine]
pub fn decay_fish(fish: String){
    
    let mut fish_json: FishStat = serde_json::from_str(fish.as_ref()).unwrap();

    fish_json.battle_power = fish_json.battle_power - 1;

    let fish_new_json = serde_json::to_string(&fish_json).unwrap();

    println!("fish {:?}", fish_new_json);
   
}


#[marine]
#[link(wasm_import_module = "host")]
extern "C" {
    fn curl(cmd: Vec<String>) -> MountedBinaryResult;
}
