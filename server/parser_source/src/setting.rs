use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::error::Error;

use crate::consts::PARTS_NAMES;


#[derive(Deserialize, Debug)]
pub struct Setting {
    #[serde(rename="useParts")]
    pub use_parts: String,
    #[serde(rename="defaults", default="HashMap::new", deserialize_with = "defaults_value_deserializer")]
    pub default_values: HashMap<String, String>,
    #[serde(rename="inputs", default="Vec::new")]
    pub input_params: Vec<String>,
}


fn defaults_value_deserializer<'de, D>(deserializer: D) -> Result<HashMap<String, String>, D::Error>
    where
        D: Deserializer<'de>
{
    let mut result = HashMap::new();
    let tmp = <serde_json::Map<String, serde_json::Value>>::deserialize(deserializer).map_err(serde::de::Error::custom)?;
    for (key, value) in tmp {
        result.insert(key, if value.is_string() { value.as_str().unwrap().to_string() } else { value.to_string() });
    }
    Ok(result)
}

pub fn load_settings(path: &str) -> Result<HashMap<String, Setting>, Box<dyn Error>> {
    let settings: serde_json::Value = serde_json::from_str(&read_to_string(path)?)?;
    let settings = settings.as_object().ok_or("settings must be object")?
        .get("commands").ok_or("commands is not exists")?.as_object().ok_or("commands must be object")?;

    let mut result = HashMap::new();

    for (parts_name, value) in settings {
        let setting: Setting = serde_json::from_value(value.clone())?;

        let use_parts_name = setting.use_parts.as_str();
        if !PARTS_NAMES.iter().any(|&x| x == use_parts_name) {
            return Result::Err(format!("{}'s useParts is not exists: {}", parts_name, use_parts_name).into());
        }

        result.insert(parts_name.clone(), setting);
    }

    Ok(result)
}
