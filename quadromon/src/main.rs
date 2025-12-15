use crate::store::StoreItem;
use crate::ui::App;
use std::io;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use crate::history::History;

mod config;
mod consts;
mod sensor_reader;
mod store;
mod ui;
mod util;
mod history;

fn main() -> Result<(), io::Error> {
    let config_mutex = Arc::new(Mutex::new(config::load_config()));

    let (tx, rx): (Sender<StoreItem>, Receiver<StoreItem>) = mpsc::channel();
    let current = store::load().expect("Failed to load store");

    let sensor_update = timer::Timer::new();

    let guard = {
        let config = config_mutex.clone();
        let s = tx.clone();

        sensor_update.schedule_repeating(chrono::Duration::seconds(3), move || {
            if let Some(flow_config) = &config.lock().unwrap().flow_sensor {
                let flow = sensor_reader::read(flow_config);

                let flow_items = store::store_result(flow);
                for x in flow_items {
                    s.send(x).unwrap();
                }
            }

            if let Some(pump_config) = &config.lock().unwrap().pump_config {
                let pump = sensor_reader::read(pump_config);
                let pump_items = store::store_result(pump);
                for x in pump_items {
                    s.send(x).unwrap();
                }
            }
        })
    };
    let h = History::from_store(current);
    let terminal = ratatui::init();
    let _ = App::run(terminal, rx, &h);
    ratatui::restore();
    drop(guard);

    Ok(())
}
