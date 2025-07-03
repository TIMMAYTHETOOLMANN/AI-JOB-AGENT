use std::collections::HashMap;
use std::fs;

pub fn load_secrets(path: &str) -> HashMap<String, String> {
    let mut secrets = HashMap::new();
    if let Ok(content) = fs::read_to_string(path) {
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') { continue; }
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim().to_string();
                let value = value.trim().trim_matches('"').to_string();
                secrets.insert(key, value);
            }
        }
    }
    secrets
}

