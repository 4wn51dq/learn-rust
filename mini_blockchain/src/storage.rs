use crate::chain::Blockchain;
use std::fs::{write, read_to_string};


pub fn save_chain(chain: &Blockchain, path: &str) -> Result<(), String> {
    let json = serde_json::to_string_pretty(chain).map_err(|e| e.to_string())?;
    write(path, json).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn load_chain(path: &str) -> Result<Blockchain, String> {
    let chain = serde_json::from_str(
        &read_to_string(path).map_err(|e| e.to_string())?
    )
        .map_err(|e| e.to_string())?;
    Ok(chain)
}