#![allow(improper_ctypes)]

use types::*;

use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::WasmLoggerBuilder;

use marine_sqlite_connector::{Connection, Error, Result, Value};

module_manifest!();

const DEFAULT_PATH: &str = "lefish_config";

pub fn main() {
    WasmLoggerBuilder::new()
        .with_log_level(log::LevelFilter::Info)
        .build()
        .unwrap();
}

#[marine]
pub fn init_config() -> LeFishResult {
    let conn = get_connection(DEFAULT_PATH);
    let res = create_tables(&conn);
    LeFishResult::from_res(res)
}

#[marine]
pub fn add(key: String, value: String) -> LeFishResult {
    let conn = get_connection(DEFAULT_PATH);

    // Check if key exist
    match get_record_by_key(&conn, key.clone()) {
        Ok(result) => {
            if result.is_none() {
                let res = add_record(&conn, key, value);
                LeFishResult::from_res(res)
            } else {
                let res = update_record(&conn, key, value);
                LeFishResult::from_res(res)
            }
        }
        Err(err) => LeFishResult::from_err_str(&err.message.unwrap()),
    }
}

#[marine]
pub fn get_value_by_key(key: String) -> String {
    let conn = get_connection(DEFAULT_PATH);
    let record = get_record_by_key(&conn, key).unwrap();

    log::info!("{:?}", record);

    if record != None {
        record.unwrap().value
    } else {
        "".to_string()
    }
}

/************************ *********************/

pub fn get_none_error() -> Error {
    Error {
        code: None,
        message: Some("Value doesn't exist".to_string()),
    }
}

pub fn get_connection(db_name: &str) -> Connection {
    let path = format!("tmp/'{}'_db.sqlite", db_name);
    Connection::open(&path).unwrap()
}

pub fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "
    create table if not exists config (
            uuid INTEGER not null primary key AUTOINCREMENT,
            key TEXT not null unique,
            value TEXT not null
        );
    ",
    )?;

    Ok(())
}

pub fn delete_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "
        drop table if exists config;
        ",
    )?;

    Ok(())
}

pub fn add_record(conn: &Connection, key: String, value: String) -> Result<()> {
    conn.execute(format!(
        "insert into config (key, value) values ('{}', '{}');",
        key, value
    ))?;

    println!(
        "insert into config (key, value) values ('{}', '{}');",
        key, value
    );

    Ok(())
}

pub fn update_record(conn: &Connection, key: String, value: String) -> Result<()> {
    conn.execute(format!(
        "
      update config 
      set value = '{}' 
      where key = '{}';
      ",
        value, key
    ))?;

    Ok(())
}

pub fn get_exact_record(conn: &Connection, key: String) -> Result<Record> {
    read_execute(conn, format!("select * from config where key = '{}';", key))
}

pub fn get_record_by_key(conn: &Connection, key: String) -> Result<Option<Record>> {
    let mut cursor = conn
        .prepare(format!("select * from config where key = '{}';", key))?
        .cursor();

    let row = cursor.next()?;
    if row != None {
        let found_record = Record::from_row(row.unwrap());
        Ok(Some(found_record.unwrap()))
    } else {
        Ok(None)
    }
}

fn read_execute(conn: &Connection, statement: String) -> Result<Record> {
    let mut cursor = conn.prepare(statement)?.cursor();
    let row = cursor.next()?.ok_or(get_none_error());
    let found_record = Record::from_row(row.unwrap_or_default());
    Ok(found_record?)
}

#[marine]
#[derive(Default, PartialEq, Debug)]
pub struct Record {
    pub uuid: i64,
    pub key: String,
    pub value: String,
    pub err_msg: String,
    pub success: bool,
}

impl Record {
    pub fn from_row(row: &[Value]) -> Result<Record> {
        let row_record = Record {
            uuid: row[0].as_integer().ok_or(get_none_error())?,
            key: row[1].as_string().ok_or(get_none_error())?.to_string(),
            value: row[2].as_string().ok_or(get_none_error())?.to_string(),
            err_msg: "".to_string(),
            success: true,
            ..Default::default()
        };

        Ok(row_record)
    }

    pub fn from_res(res: Result<Record>) -> Record {
        match res {
            Ok(v) => v,
            Err(e) => {
                let mut res_data: Record = Default::default();
                res_data.err_msg = e.to_string();
                res_data.success = false;
                res_data
            }
        }
    }
}
