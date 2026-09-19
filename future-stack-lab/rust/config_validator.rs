use std::collections::HashMap;

fn parse_config(input: &str) -> Result<HashMap<String, String>, String> {
    let mut config = HashMap::new();

    for (index, raw_line) in input.lines().enumerate() {
        let line = raw_line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| format!("Line {} is missing '='", index + 1))?;

        let key = key.trim();
        let value = value.trim();

        if key.is_empty() {
            return Err(format!("Line {} has an empty key", index + 1));
        }

        if value.is_empty() {
            return Err(format!("Line {} has an empty value", index + 1));
        }

        if config.insert(key.to_string(), value.to_string()).is_some() {
            return Err(format!("Duplicate key '{}' found", key));
        }
    }

    Ok(config)
}

fn require<'a>(
    config: &'a HashMap<String, String>,
    key: &str,
) -> Result<&'a str, String> {
    config
        .get(key)
        .map(String::as_str)
        .ok_or_else(|| format!("Required key '{}' is missing", key))
}

fn main() {
    let raw = r#"
# FutureStack sample configuration
APP_NAME=portfolio-api
PORT=8080
ENVIRONMENT=development
"#;

    match parse_config(raw) {
        Ok(config) => {
            println!("Configuration is valid.");

            for key in ["APP_NAME", "PORT", "ENVIRONMENT"] {
                match require(&config, key) {
                    Ok(value) => println!("{}={}", key, value),
                    Err(error) => eprintln!("Validation error: {}", error),
                }
            }
        }
        Err(error) => eprintln!("Configuration error: {}", error),
    }
}
