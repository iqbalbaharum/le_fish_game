use marine_rs_sdk::marine;
use marine_sqlite_connector::Result;
//
#[marine]
pub struct LeFishResult {
    pub success: bool,
    pub err_msg: String,
}

impl LeFishResult {
    pub fn from_res(res: Result<()>) -> LeFishResult {
        match res {
            Ok(_v) => LeFishResult {
                success: true,
                err_msg: "".into(),
            },
            Err(e) => LeFishResult {
                success: false,
                err_msg: e.to_string(),
            },
        }
    }

    pub fn from_err_str(e: &str) -> LeFishResult {
        LeFishResult {
            success: false,
            err_msg: e.to_string(),
        }
    }
}
