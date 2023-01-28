use marine_rs_sdk::marine;
use marine_sqlite_connector::Result;
//
#[marine]
pub struct ConfigResult {
    pub success: bool,
    pub err_msg: String,
}

impl ConfigResult {
    pub fn from_res(res: Result<()>) -> ConfigResult {
        match res {
            Ok(_v) => ConfigResult {
                success: true,
                err_msg: "".into(),
            },
            Err(e) => ConfigResult {
                success: false,
                err_msg: e.to_string(),
            },
        }
    }

    pub fn from_err_str(e: &str) -> ConfigResult {
        ConfigResult {
            success: false,
            err_msg: e.to_string(),
        }
    }
}
