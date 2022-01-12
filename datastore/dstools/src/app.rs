use datastorelib::datastore::Datastore;
use rand::{
    distributions::{Distribution, Uniform},
    rngs::ThreadRng,
};
use std::path::Path;
use std::string::String;
use tui::widgets::ListState;

const TASKS: [&str; 24] = [
    "Item1", "Item2", "Item3", "Item4", "Item5", "Item6", "Item7", "Item8", "Item9", "Item10",
    "Item11", "Item12", "Item13", "Item14", "Item15", "Item16", "Item17", "Item18", "Item19",
    "Item20", "Item21", "Item22", "Item23", "Item24",
];

const MENU: [&str; 3] = ["Inspector", "Menu item 1", "Menu item 2"];

const LOGS: [(&str, &str); 26] = [
    ("Event1", "INFO"),
    ("Event2", "INFO"),
    ("Event3", "CRITICAL"),
    ("Event4", "ERROR"),
    ("Event5", "INFO"),
    ("Event6", "INFO"),
    ("Event7", "WARNING"),
    ("Event8", "INFO"),
    ("Event9", "INFO"),
    ("Event10", "INFO"),
    ("Event11", "CRITICAL"),
    ("Event12", "INFO"),
    ("Event13", "INFO"),
    ("Event14", "INFO"),
    ("Event15", "INFO"),
    ("Event16", "INFO"),
    ("Event17", "ERROR"),
    ("Event18", "ERROR"),
    ("Event19", "INFO"),
    ("Event20", "INFO"),
    ("Event21", "WARNING"),
    ("Event22", "INFO"),
    ("Event23", "INFO"),
    ("Event24", "WARNING"),
    ("Event25", "INFO"),
    ("Event26", "INFO"),
];

const EVENTS: [(&str, u64); 24] = [
    ("B1", 9),
    ("B2", 12),
    ("B3", 5),
    ("B4", 8),
    ("B5", 2),
    ("B6", 4),
    ("B7", 5),
    ("B8", 9),
    ("B9", 14),
    ("B10", 15),
    ("B11", 1),
    ("B12", 0),
    ("B13", 4),
    ("B14", 6),
    ("B15", 4),
    ("B16", 6),
    ("B17", 4),
    ("B18", 7),
    ("B19", 13),
    ("B20", 8),
    ("B21", 11),
    ("B22", 9),
    ("B23", 3),
    ("B24", 5),
];

#[derive(Clone)]
pub struct RandomSignal {
    distribution: Uniform<u64>,
    rng: ThreadRng,
}

impl RandomSignal {
    pub fn new(lower: u64, upper: u64) -> RandomSignal {
        RandomSignal {
            distribution: Uniform::new(lower, upper),
            rng: rand::thread_rng(),
        }
    }
}

impl Iterator for RandomSignal {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        Some(self.distribution.sample(&mut self.rng))
    }
}

#[derive(Clone)]
pub struct SinSignal {
    x: f64,
    interval: f64,
    period: f64,
    scale: f64,
}

impl SinSignal {
    pub fn new(interval: f64, period: f64, scale: f64) -> SinSignal {
        SinSignal {
            x: 0.0,
            interval,
            period,
            scale,
        }
    }
}

impl Iterator for SinSignal {
    type Item = (f64, f64);
    fn next(&mut self) -> Option<Self::Item> {
        let point = (self.x, (self.x * 1.0 / self.period).sin() * self.scale);
        self.x += self.interval;
        Some(point)
    }
}

pub struct AppState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> AppState<'a> {
    pub fn new(titles: Vec<&'a str>) -> AppState {
        AppState { titles, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn with_items_selected(items: Vec<T>, index: usize) -> StatefulList<T> {
        let mut sl = StatefulList {
            state: ListState::default(),
            items,
        };
        if sl.items.len() > index {
            sl.state.select(Some(index));
        }
        return sl;
    }

    pub fn select(&mut self, index: usize) {
        if self.items.len() > index {
            self.state.select(Some(index));
        } else {
            self.state.select(None);
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

pub struct Signal<S: Iterator> {
    source: S,
    pub points: Vec<S::Item>,
    tick_rate: usize,
}

impl<S> Signal<S>
where
    S: Iterator,
{
    fn on_tick(&mut self) {
        for _ in 0..self.tick_rate {
            self.points.remove(0);
        }
        self.points
            .extend(self.source.by_ref().take(self.tick_rate));
    }
}

pub struct Signals {
    pub sin1: Signal<SinSignal>,
    pub sin2: Signal<SinSignal>,
    pub window: [f64; 2],
}

impl Signals {
    fn on_tick(&mut self) {
        self.sin1.on_tick();
        self.sin2.on_tick();
        self.window[0] += 1.0;
        self.window[1] += 1.0;
    }
}

#[derive(PartialEq)]
pub enum InputContext {
    DataStorePath,
    Initial,
}

pub struct Server<'a> {
    pub name: &'a str,
    pub location: &'a str,
    pub coords: (f64, f64),
    pub status: &'a str,
}

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub apps: AppState<'a>,
    pub entered_text: String,
    pub ds_base_path: String,
    pub context: InputContext,
    pub should_submit: bool,
    pub should_tab: bool,
    pub should_f4: bool,
    pub should_f5: bool,
    pub should_f6: bool,
    pub should_f7: bool,
    pub should_f8: bool,
    pub should_f9: bool,
    pub data_store: Option<Datastore>,
    pub menu: StatefulList<&'a str>,
    pub show_chart: bool,
    pub progress: f64,
    pub sparkline: Signal<RandomSignal>,
    pub tasks: StatefulList<&'a str>,
    pub logs: StatefulList<(&'a str, &'a str)>,
    pub signals: Signals,
    pub barchart: Vec<(&'a str, u64)>,
    pub servers: Vec<Server<'a>>,
    pub enhanced_graphics: bool,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool) -> App<'a> {
        let mut rand_signal = RandomSignal::new(0, 100);
        let sparkline_points = rand_signal.by_ref().take(300).collect();
        let mut sin_signal = SinSignal::new(0.2, 3.0, 18.0);
        let sin1_points = sin_signal.by_ref().take(100).collect();
        let mut sin_signal2 = SinSignal::new(0.1, 2.0, 10.0);
        let sin2_points = sin_signal2.by_ref().take(200).collect();
        let apps = AppState::new(MENU.to_vec());
        App {
            title,
            should_quit: false,
            apps: apps,
            entered_text: String::new(),
            ds_base_path: String::from("/"),
            should_submit: false,
            should_tab: false,
            should_f4: false,
            should_f5: false,
            should_f6: false,
            should_f7: false,
            should_f8: false,
            should_f9: false,
            context: InputContext::Initial,
            show_chart: true,
            data_store: None,
            progress: 0.0,
            sparkline: Signal {
                source: rand_signal,
                points: sparkline_points,
                tick_rate: 1,
            },
            menu: StatefulList::with_items_selected(MENU.to_vec(), 0),
            tasks: StatefulList::with_items(TASKS.to_vec()),
            logs: StatefulList::with_items(LOGS.to_vec()),
            signals: Signals {
                sin1: Signal {
                    source: sin_signal,
                    points: sin1_points,
                    tick_rate: 5,
                },
                sin2: Signal {
                    source: sin_signal2,
                    points: sin2_points,
                    tick_rate: 10,
                },
                window: [0.0, 20.0],
            },
            barchart: EVENTS.to_vec(),
            servers: vec![
                Server {
                    name: "NorthAmerica-1",
                    location: "New York City",
                    coords: (40.71, -74.00),
                    status: "Up",
                },
                Server {
                    name: "Europe-1",
                    location: "Paris",
                    coords: (48.85, 2.35),
                    status: "Failure",
                },
                Server {
                    name: "SouthAmerica-1",
                    location: "São Paulo",
                    coords: (-23.54, -46.62),
                    status: "Up",
                },
                Server {
                    name: "Asia-1",
                    location: "Singapore",
                    coords: (1.35, 103.86),
                    status: "Up",
                },
            ],
            enhanced_graphics,
        }
    }

    pub fn activate_one(&mut self) {
        self.context = InputContext::DataStorePath;
        self.menu.select(0);
    }

    pub fn activate_two(&mut self) {
        self.context = InputContext::Initial;
        self.menu.select(1);
    }
    pub fn activate_three(&mut self) {
        self.context = InputContext::Initial;
        self.menu.select(2);
    }

    pub fn next_app(&mut self) {
        self.menu.next()
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn delete(&mut self) {
        if self.entered_text.len() > 0 {
            /* let mut string = self.entered_text.to_string();
            let _ = string.pop();
            self.entered_text = string.to_owned().as_str() */
            let _ = self.entered_text.pop();
        } else if self.entered_text.len() == 0 && self.ds_base_path.len() > 0 {
            let mut ps = Path::new(self.ds_base_path.as_str());
            let parent_path = ps.parent().unwrap();
            let mut path_string = parent_path.to_str().unwrap().to_string();
            path_string.push('/');
            self.ds_base_path = path_string;
        }
    }

    pub fn tab(&mut self) {
        self.should_tab = true;
    }

    pub fn submit(&mut self) {
        self.should_submit = true
    }

    pub fn on_up(&mut self) {
        self.tasks.previous();
    }

    pub fn on_down(&mut self) {
        self.tasks.next();
    }

    pub fn on_right(&mut self) {
        self.apps.next();
    }

    pub fn on_left(&mut self) {
        self.apps.previous();
    }

    pub fn on_key(&mut self, c: char) {
        let mut string = self.entered_text.to_string();
        string.push(c);
        self.entered_text = string;
    }

    pub fn on_tick(&mut self) {
        // Update progress
        self.progress += 0.001;
        if self.progress > 1.0 {
            self.progress = 0.0;
        }

        self.sparkline.on_tick();
        self.signals.on_tick();

        let log = self.logs.items.pop().unwrap();
        self.logs.items.insert(0, log);

        let event = self.barchart.pop().unwrap();
        self.barchart.insert(0, event);
    }
}
