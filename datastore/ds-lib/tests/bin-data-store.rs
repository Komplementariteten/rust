#[cfg(test)]
mod tests {
    #[test]
    fn test_bin_handling() {
        use serde_json::{Result, Value};

        let json_srt = "{\"name\": 12, \"value\": 1234E-10 }";

        let v: Value = serde_json::from_str(json_srt)?;

    }
}