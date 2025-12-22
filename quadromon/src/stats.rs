use crate::consts::{FLOW_SPEED_NAME, PUMP_PWM_NAME};
use std::collections::HashMap;

pub(crate) struct Stats {}

impl Stats {
    pub(crate) fn pump_flow_rel(h: &HashMap<String, Vec<u64>>) -> f32 {
        let latest_flow;
        let latest_pump;

        if let Some(l) = Self::get_latest(FLOW_SPEED_NAME, h) {
            latest_flow = l;
        } else {
            return -1.;
        }
        if let Some(l) = Self::get_latest(PUMP_PWM_NAME, h) {
            latest_pump = l;
        } else {
            return -1.;
        }

        latest_pump as f32 / latest_flow as f32
    }

    fn get_latest(name: &str, h: &HashMap<String, Vec<u64>>) -> Option<u64> {
        if h.contains_key(name) {
            let flow = h.get(name).unwrap();
            return Some(flow.iter().last().unwrap().clone());
        }
        None
    }

    pub(crate) fn avg_flow(h: &HashMap<String, Vec<u64>>) -> f32 {
        let mut sum: u64 = 0;
        if h.contains_key(FLOW_SPEED_NAME) {
            for x in h[FLOW_SPEED_NAME].clone() {
                sum += x
            }
            return sum as f32 / h.get(FLOW_SPEED_NAME).iter().len() as f32;
        }

        -1.
    }
}
