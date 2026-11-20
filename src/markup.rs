use std::collections::HashMap;
use std::{fs, io};

pub fn parse(file: &str) -> io::Result<HashMap<String, String>> {
    let mut map: HashMap<String, String> = HashMap::new();
    let data = fs::read_to_string(file)?;

    for ln in data.lines() {
        let needed = match ln.split_once("//") {
            Some((code, _comment)) => code,
            _ => ln,
        };
        let line = needed.trim();
        if line.is_empty() || line.starts_with("//") {
            continue;
        }

        let Some(stmt) = line.strip_suffix(';') else {
            continue;
        };

        if let Some((k, v)) = stmt.split_once("=") {
            if k.trim().is_empty() || v.trim().is_empty() {
                continue;
            }
            map.insert(k.trim().replace(' ', "_"), v.trim().replace(' ', "_"));
        }
    }
    Ok(map)
}
