use crate::StorageValue;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct Storage {
    pub hm: HashMap<String, StorageValue>,
}

impl Storage {
    pub fn new() -> Self {
        Self { hm: HashMap::new() }
    }

    pub fn new_from_rdb_file(path: &str) -> Self {
        let file = File::open(path);
        let mut hm: HashMap<String, StorageValue> = HashMap::new();
        match file {
            Ok(inner) => {
                let mut buf: [u8; 1024] = [0; 1024];
                let mut rdr = BufReader::new(inner).read(&mut buf).unwrap();
                let mut fb_pos = buf.iter().position(|&b| b == 0xfb).unwrap(); // Indicates that hash table size information.
                fb_pos += 1; // To read the total key-value pairs
                let key_value_pairs = buf[fb_pos];
                fb_pos += 3; // To skip the total key-value pairs with expiry date

                let fc_positions: Vec<usize> = buf
                    .iter()
                    .enumerate()
                    .filter_map(|(index, &byte)| if byte == 0xfc { Some(index) } else { None })
                    .collect();

                if fc_positions.len() > 0 {
                    for mut fc_pos in fc_positions {
                        let now = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .expect("Time went backwards")
                            .as_millis();

                        fc_pos += 1; // To move past the fc_position
                        let timestamp = to_timestamp(&buf[fc_pos..fc_pos + 8]);
                        fc_pos += 9; // To move past the timestamp and the 00 hex
                        let key_len = buf[fc_pos];
                        fc_pos += 1; // To move past the 1-byte flag that specifies the key's type and encoding.
                        let key_range = fc_pos..(fc_pos + key_len as usize);
                        let key = &buf[key_range];
                        fc_pos += 1; // To move past the 1-byte flag that specifies the value's type and encoding.
                        fc_pos += key_len as usize; // We can move the position past the key now.

                        let value_len = buf[fc_pos - 1];
                        let value_range = fc_pos..(fc_pos + value_len as usize);
                        let value = &buf[value_range];
                        fc_pos += value_len as usize; // We can move the position past the value now.

                        let k = String::from_utf8_lossy(key).to_string();
                        println!("key {} key_len {} timestamp {}", k, key_len, timestamp);

                        let v = String::from_utf8_lossy(value).to_string();

                        let mut sv;
                        if now > timestamp {
                            sv = StorageValue::new(String::from("$-1\r\n"), -1);
                        } else {
                            sv = StorageValue::new(v, -1);
                        }
                        hm.insert(k, sv);
                    }
                } else {
                    for i in 0..key_value_pairs {
                        let key_len = buf[fb_pos];
                        fb_pos += 1; // Skips the 1-byte flag that specifies the key's type and encoding.
                        let key_range = fb_pos..(fb_pos + key_len as usize);
                        let key = &buf[key_range];
                        fb_pos += 1; // Skips the 1-byte flag that specifies the value's type and encoding.
                        fb_pos += key_len as usize; // We can move the position past the key now.
                        let value_len = buf[fb_pos - 1];
                        let value_range = fb_pos..(fb_pos + value_len as usize);
                        let value = &buf[value_range];
                        fb_pos += value_len as usize; // We can move the position past the value now.
                        let k = String::from_utf8_lossy(key).to_string();
                        let v = String::from_utf8_lossy(value).to_string();
                        let sv = StorageValue::new(v, -1);
                        hm.insert(k, sv);
                        fb_pos += 1 // Moves on to the next key-value pair
                    }
                }
            }
            Err(e) => {
                eprintln!("error {e}");
            }
        }
        Self { hm }
    }

    pub fn set(&mut self, k: String, v: String, px: Option<i64>) {
        match px {
            Some(px) => self.hm.insert(k, StorageValue::new(v, px)),
            None => self.hm.insert(k, StorageValue::new(v, -1)),
        };
    }

    pub fn get(&self, k: &str) -> Option<StorageValue> {
        self.hm.get(k).cloned()
    }

    pub fn get_all_keys(&self) -> Option<Vec<&str>> {
        let mut result: Vec<&str> = Vec::new();
        let keys = self.hm.keys();
        if keys.len() == 0 {
            return None;
        }

        for key in keys {
            result.push(key);
        }
        Some(result)
    }
}

fn to_timestamp(data: &[u8]) -> u128 {
    let hex_string: String = data.iter().map(|byte| format!("{:02x}", byte)).collect();
    let reversed_hex: String = hex_string
        .as_bytes()
        .chunks(2)
        .rev()
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .collect();
    u128::from_str_radix(&reversed_hex, 16).expect("Invalid hex string")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage() {
        let mut storage = Storage::new();
        let k = "test-key".to_string();
        let v = StorageValue::new("test-value".to_string(), 100);
        storage.set(k.clone(), "test-value".to_string(), None);

        let actual = storage.get(&k).unwrap().value;
        let expected = Some(v).unwrap().value;
        assert_eq!(actual, expected)
    }
}
