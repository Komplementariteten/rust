use crate::store::StoreItem;
use crate::ui::App;
use std::io;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use crate::data_hub::DataHub;

mod config;
mod consts;
mod sensor_reader;
mod store;
mod ui;
mod util;
mod data_hub;
mod stats;

fn main() -> Result<(), io::Error> {
    let config_mutex = Arc::new(Mutex::new(config::load_config()));

    let (tx, rx): (Sender<StoreItem>, Receiver<StoreItem>) = mpsc::channel();
    let sensor_update = timer::Timer::new();
    let hub_mutex = Arc::new(Mutex::new(DataHub::new(tx)));

    let guard = {
        let config = config_mutex.clone();
        let hub = hub_mutex.clone();
        sensor_update.schedule_repeating(chrono::Duration::seconds(3), move || {
            if let Some(flow_config) = &config.lock().unwrap().flow_sensor {
                let flow = sensor_reader::read(flow_config);
                hub.lock().unwrap().update(flow);
            }

            if let Some(pump_config) = &config.lock().unwrap().pump_config {
                let pump = sensor_reader::read(pump_config);
                hub.lock().unwrap().update(pump);
            }
        })
    };
    let terminal = ratatui::init();
    let _ = App::run(terminal, rx, &hub_mutex);
    ratatui::restore();
    drop(guard);

    Ok(())
}
