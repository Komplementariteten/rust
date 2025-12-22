use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use dirs;
use toml;
use crate::consts::{APP_CONFIG_FILE, APP_NAME, FLOW_SPEED_NAME, PUMP_PWM_NAME, PUMP_SPEED_NAME, TEMPERATURE_1_NAME, TEMP_SENSOR_NAME};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Config {
    statistics: Option<StatisticsConfig>,
    pub flow_sensor: Option<SensorConfig>,
    pub pump_config: Option<SensorConfig>
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct StatisticsConfig{
    history: u32,
    history_file: PathBuf,
    evaluations: Vec<Evaluation>
}
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct SensorConfig {
    pub module_name: String,
    pub base_path: PathBuf,
    pub sensors: Vec<Sensor>
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Sensor {
    pub name: String,
    pub file: PathBuf,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct  Evaluation {
    settings: HashMap<String, u32>,
    name: String,
    data_range_by_time: Option<Duration>,
    data_range_by_count: usize
}

impl SensorConfig {
    fn new_flow_sensor() -> SensorConfig {
        SensorConfig {
            module_name: "quadro".to_string(),
            base_path: PathBuf::from("/sys/class/hwmon/hwmon4/"),
            sensors: vec![Sensor {
               name: FLOW_SPEED_NAME.to_string(),
                file: PathBuf::from("fan5_input"),
            }, Sensor {
                name: TEMP_SENSOR_NAME.to_string(),
                file: PathBuf::from("temp1_input"),
            } ]
        }
    }
    fn new_pump_sensor() -> SensorConfig
        {
            SensorConfig{
                module_name: "nct6687".to_string(),
                base_path: PathBuf::from("/sys/class/hwmon/hwmon7/"),
                sensors: vec![Sensor {
                    name: PUMP_SPEED_NAME.to_string(),
                    file: PathBuf::from("fan2_input"),
                }]
            }
        }
}

impl Config {
    fn new() -> Config {
        Config {
            statistics: None,
            flow_sensor: Some(SensorConfig::new_flow_sensor()),
            pump_config: Some(SensorConfig::new_pump_sensor())
        }
    }
}

pub(crate) fn load_config() -> Config{
    let sys_config_dir = dirs::config_local_dir().unwrap_or(dirs::home_dir().expect("Failed to determine home directory."));
    let config_dir = sys_config_dir.join(APP_NAME);
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).expect("Failed to create config directory");
    }
    let config_file = config_dir.join(APP_CONFIG_FILE);
    if let Ok(true) = fs::exists(&config_file) {
        let file_text = fs::read_to_string(&config_file).expect("Failed to read config file");
        return toml::from_str(&file_text).expect("Failed to parse config file");
    }

    let config = Config::new();
    let file_text = toml::to_string(&config).expect("Failed to serialize config file");
    fs::write(&config_file, &file_text).expect("Failed to write config file");
    config
}

