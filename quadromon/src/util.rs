use crate::consts::{
    PULSE_PER_LITER, PUMP_PWM_NAME, PUMP_SPEED_NAME, TEMPERATURE_1_NAME, TEMP_SENSOR_NAME,
};
use std::collections::HashMap;

pub(crate) struct Util {}

impl Util {
    pub(crate) fn format(name: &str, value: u64) -> String {
        match name {
            FLOW_SPEED_NAME => format!("{}", (value * 2) as f32 / PULSE_PER_LITER),
            TEMPERATURE_1_NAME => format!("{}", value as f32 / 10000.),
            TEMP_SENSOR_NAME => format!("{}", value as f32 / 10000.),
            _ => "-1.".to_string(),
        }
    }
    pub(crate) fn format_u32(name: &str, value: u32) -> String {
        let u64_v = value as u64;
        Util::format(name, u64_v)
    }
    pub(crate) fn format_map(map: &HashMap<String, u64>) -> HashMap<String, String> {
        HashMap::new()
    }

    pub(crate) fn get_ui_name(modname: &str) -> String {
        match modname.trim() {
            TEMP_SENSOR_NAME => TEMPERATURE_1_NAME.to_string(),
            PUMP_SPEED_NAME => PUMP_PWM_NAME.to_string(),
            _ => modname.to_string(),
        }
    }
}
