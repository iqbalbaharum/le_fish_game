#![allow(improper_ctypes)]

use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::MountedBinaryResult;
use marine_rs_sdk::WasmLoggerBuilder;
use serde_json::json;

module_manifest!();

pub struct Mint{
    id: String
    to: String
    blockNo: String
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
    let curl_string = String::from_utf8(response.stdout)?;
    let curl_json:Mint = serde_json::from_str(curl_string).unwrap();
    
    for(key, value) in curl_json.as_object().unwrap().iter() {
        
    }
}

#[marine]
#[link(wasm_import_module = "host")]
extern "C" {
    fn curl(cmd: Vec<String>) -> MountedBinaryResult;
}
