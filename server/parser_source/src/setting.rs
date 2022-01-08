use serde::{ Deserialize, Deserializer };
use std::collections::HashMap;
use std::fs::read_to_string;
use std::error::Error;

use crate::consts::PARTS_NAMES;


#[derive(Debug, Default, Deserialize)]
pub struct Settings {
    #[serde(rename="commands", default="HashMap::new")]
    pub command_to_setting: HashMap<String, GraphicPartsSetting>,
    #[serde(rename="initialText", default="String::new", deserialize_with = "initial_text_deserializer")]
    pub initial_text: String,
}

fn initial_text_deserializer<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>
{
    let tmp = <serde_json::Value>::deserialize(deserializer).map_err(serde::de::Error::custom)?;
    if let Some(s) = tmp.as_str() {
        Ok(s.to_string())
    } else if let Some(v) = tmp.as_array() {
        for x in v { if !x.is_string() { return Err("initial text must be string or [string]").map_err(serde::de::Error::custom) } }
        Ok(v.iter()
            .map(|x|x.as_str().unwrap())
            .collect::<Vec<_>>()
            .join(" "))
    } else {
        Err("initial text must be string or [string]").map_err(serde::de::Error::custom)
    }
}


#[derive(Debug, Deserialize)]
pub struct GraphicPartsSetting {
    #[serde(rename="useElem")]
    pub use_elem: String,
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

pub fn load_settings(path: &str) -> Result<Settings, Box<dyn Error>> {
    let settings: Settings = serde_json::from_str(&read_to_string(path)?)?;

    for (elem_name, setting) in &settings.command_to_setting {
        if !PARTS_NAMES.contains(setting.use_elem.as_str()) {
            return Result::Err(format!("{}'s useElem is invalid: {}", elem_name, setting.use_elem).into());
        }
    }

    Ok(settings)
}
