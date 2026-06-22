use crate::chain::Blockchain;
use std::fs::{write(path, contents), read_to_string(path)};


pub fn save_chain(chain: &Blockchain, path: &str) -> Result<(), String> {

}

pub fn load_chain(path: &str) -> Result<Blockchain, String> {

}