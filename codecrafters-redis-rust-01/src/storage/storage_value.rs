use std::thread::sleep;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct StorageValue {
    pub value: String,
    pub timestamp: Instant,
    pub px: i64,
}

impl StorageValue {
    pub fn new(value: String, px: i64) -> Self {
        Self {
            value,
            timestamp: Instant::now(),
            px,
        }
    }

    pub fn is_expired(&self) -> bool {
        if self.px == -1 {
            return false;
        }
        if self.timestamp.elapsed().as_millis() < self.px as u128 {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn test_new() {
        let sv = StorageValue::new("test-value".to_string(), 100);
        assert_eq!(sv.value, String::from("test-value"));
        assert_eq!(sv.px, 100);
    }

    #[test]
    fn test_is_expired() {
        let sv = StorageValue::new("test-value".to_string(), 1);
        sleep(Duration::from_millis(2));
        assert_eq!(sv.is_expired(), true);
    }
}
