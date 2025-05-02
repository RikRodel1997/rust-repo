fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut result = String::new();
    let min_len = strs.iter().map(|s| s.chars().count()).min().unwrap();

    for i in 0..min_len {
        let mut chars = Vec::with_capacity(min_len);
        for str in &strs {
            if let Some(c) = str.chars().nth(i) {
                chars.push(c);
            }
        }
        let first = chars.first().unwrap();
        if chars.iter().all(|c| c == first) {
            result.push_str(first.to_string().as_str());
        } else {
            return result;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_common_prefix() {
        let result = longest_common_prefix(vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ]);
        assert_eq!(result, "fl".to_string());
    }

    #[test]
    fn has_no_common_prefix() {
        let result = longest_common_prefix(vec![
            "dog".to_string(),
            "racecar".to_string(),
            "car".to_string(),
        ]);
        assert_eq!(result, "".to_string());
    }
}
