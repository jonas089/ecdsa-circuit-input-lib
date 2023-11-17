use rusqlite;
use rusqlite::{Connection, Result};
use std::path::PathBuf;
use serde_json;

#[derive(Debug)]
pub struct Response{
    pub uid: String,
    pub sk: String
}
impl Response{
    pub fn deserialize(&self) -> Vec<u8>{
        serde_json::from_str(&self.sk)
            .expect("[Error] Failed to parse sk as bytes!")
    }
}

pub struct StoreManager{
    pub path: PathBuf
}

impl StoreManager{
    pub fn init(&self) -> Result<()>{
        let conn: Connection = Connection::open(&self.path)?;
        conn.execute(
            // sk: secret key
            "CREATE TABLE IF NOT EXISTS ecdsa (
                      id INTEGER PRIMARY KEY,
                      uid TEXT NOT NULL,
                      sk TEXT NOT NULL
                      )",
            [],
        )?;
        Ok(())
    }
    
    pub fn insert_key(&self, uid: String, key: Vec<u8>) -> Result<()>{
        let sk: String = serde_json::to_string(&key)
            .expect("[Error] Failed to serialize key!");
        let conn: Connection = Connection::open(&self.path)?;
        conn.execute(
            "INSERT INTO ecdsa (uid, sk) VALUES (?1, ?2)",
            &[&uid, &sk]
        )?;
        Ok(())
    }

    pub fn get_keys(&self) -> Result<Vec<Response>>{
        let conn = Connection::open(&self.path)?;
        let mut state = conn.prepare("SELECT uid, sk FROM ecdsa")?;
        let data_iter = state.query_map([], |row| {
            Ok(Response {
                uid: row.get(0)?,
                sk: row.get(1)?,
            })
        })?;
        let mut result = Vec::new();
        for data in data_iter {
            result.push(data?);
        }
        Ok(result)
    }

    pub fn get_key_by_uid(&self, uid: String) -> Result<Response>{
        let conn = Connection::open(&self.path)?;
        let mut stmt = conn
            .prepare("SELECT uid, sk FROM ecdsa WHERE uid = ?1 LIMIT 1")?;
        // Use query_row and check for QueryReturnedNoRows error
        match stmt.query_row(&[&uid], |row| {
            Ok(Response {
                uid: row.get(0)?,
                sk: row.get(1)?,
            })
        }) {
            Ok(key) => Ok(key),
            Err(err) => Err(err),
        }
    }
}