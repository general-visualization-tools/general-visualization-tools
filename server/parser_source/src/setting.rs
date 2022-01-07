use serde::{Serialize, Deserialize, Deserializer};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::error::Error;

use crate::consts::PARTS_NAMES;


#[derive(Debug)]
pub struct Settings {
    pub parts: HashMap<String, PartsSetting>,
    pub camera: CameraSetting,
}

fn default_0i32() -> i32 { 0 }
fn default_1000usize() -> usize { 1000 }

#[derive(Serialize, Deserialize, Debug)]
pub struct CameraSetting {
    #[serde(default="default_0i32")]
    x: i32,
    #[serde(default="default_0i32")]
    y: i32,
    #[serde(default="default_1000usize")]
    w: usize,
    #[serde(default="default_1000usize")]
    h: usize,
}

#[derive(Deserialize, Debug)]
pub struct PartsSetting {
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

impl Default for Settings { fn default() -> Self { Self { parts: HashMap::new(), camera: CameraSetting::default() } } }
impl Default for CameraSetting { fn default() -> Self { Self { x: 0, y: 0, w: 1000, h: 1000 } } }

pub fn load_settings(path: &str) -> Result<Settings, Box<dyn Error>> {
    let mut settings = Settings::default();

    let raw_settings: serde_json::Map<String, serde_json::Value> = serde_json::from_str(&read_to_string(path)?)?;
    // let raw_settings: serde_json::Value = serde_json::from_str(&read_to_string(path)?)?;
    // let raw_settings = raw_settings.as_object().ok_or("settings must be object")?;


    // load parts settings
    {
        let parts_settings = raw_settings.get("commands").ok_or("commands is not exists")?.as_object().ok_or("commands must be object")?;
        let mut checked_parts_settings = HashMap::new();
        for (parts_name, value) in parts_settings {
            let setting: PartsSetting = serde_json::from_value(value.clone())?;

            let use_parts_name = setting.use_parts.as_str();
            if !PARTS_NAMES.iter().any(|&x| x == use_parts_name) {
                return Result::Err(format!("{}'s useParts is not exists: {}", parts_name, use_parts_name).into());
            }

            checked_parts_settings.insert(parts_name.clone(), setting);
        }
        settings.parts = checked_parts_settings;
    }

    // load camera settings
    settings.camera = if let Some(v) = raw_settings.get("camera") { serde_json::from_value(v.clone())? } else { CameraSetting::default() };

    Ok(settings)
}
