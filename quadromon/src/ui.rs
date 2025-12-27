use crate::consts::{APP_NAME, AVG_FLOW, HIST_SIZE, PUMP_FLOW_REL};
use crate::data_hub::DataHub;
use crate::stats::Stats;
use crate::store::StoreItem;
use crate::util::Util;
use chrono::{DateTime, Local};
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Rect};
use ratatui::prelude::*;
use ratatui::style::palette::tailwind::SLATE;
use ratatui::style::{Style};
use ratatui::text::{Line, Text, ToSpan};
use ratatui::widgets::{
    Block, Borders, HighlightSpacing, List, ListItem, ListState, Paragraph, Sparkline,
};
use ratatui::{layout::Layout, DefaultTerminal, Frame};
use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::time::Duration;

struct UiState {
    latest_update: HashMap<String, String>,
}

impl UiState {
    pub fn new(max_values: HashMap<String, u64>) -> UiState {
        UiState {
            latest_update: Util::format_map(max_values),
        }
    }
}

pub struct App {
    state: UiState,
    current_value_state: ListState,
    h: Arc<Mutex<DataHub>>,
}

impl App {
    fn new(h: &Arc<Mutex<DataHub>>) -> App {
        App {
            state: UiState::new(h.lock().unwrap().max_values.clone()),
            current_value_state: ListState::default(),
            h: h.clone(),
        }
    }

    fn update_state(&mut self, v: StoreItem) {
        let formated = Util::format_u32(&v.item_name, v.value);
        self.state
            .latest_update
            .entry(v.item_name.clone())
            .and_modify(|x| *x = formated.clone())
            .or_insert(formated);
        self.update_stats();
    }

    fn update_stats(&mut self) {
        let h = self.h.lock().unwrap().get_history(HIST_SIZE);
        let rel = Stats::pump_flow_rel(&h);
        let avg = Stats::avg_flow(&h);
        self.state
            .latest_update
            .entry(PUMP_FLOW_REL.to_string())
            .and_modify(|x| *x = rel.to_string())
            .or_insert(avg.to_string());
        self.state
            .latest_update
            .entry(AVG_FLOW.to_string())
            .and_modify(|x| *x = avg.to_string())
            .or_insert(avg.to_string());
    }

    fn draw(&mut self, f: &mut Frame) {
        f.render_widget(self, f.area())
    }

    pub(crate) fn run(
        mut terminal: DefaultTerminal,
        rx: Receiver<StoreItem>,
        h: &Arc<Mutex<DataHub>>,
    ) {
        let mut app = App::new(h);
        loop {
            if let Ok(update) = rx.try_recv() {
                app.update_state(update);
            }
            terminal
                .draw(|f| {
                    app.draw(f);
                })
                .expect("ui drawing failed");
            if let Ok(ok) = event::poll(Duration::from_millis(10))
                && ok
            {
                match event::read().expect("event failed") {
                    Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                        KeyCode::Esc => {
                            h.lock().unwrap().sync();
                            return
                        },
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }
    fn render_current_values(&mut self, area: Rect, buf: &mut Buffer) {
        let values = Block::new()
            .title(Line::raw("Current values").centered())
            .borders(Borders::ALL);
        let list_items: Vec<ListItem> = self
            .state
            .latest_update
            .iter()
            .map(|(k, v)| {
                let mut text = Text::default();
                text.extend([k.clone().italic(), v.to_span().bold(), Span::raw("")]);
                ListItem::new(text)
            })
            .collect();

        let list = List::new(list_items)
            .block(values)
            .highlight_style(Style::new().bg(SLATE.c800))
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);
        StatefulWidget::render(list, area, buf, &mut self.current_value_state);
    }

    fn render_spark_line(&mut self, areas: &Rect, buf: &mut Buffer, name: &str) {
        let h = self.h.lock().unwrap().get_history(HIST_SIZE);
        if !h.contains_key(name) {
            return;
        }
        let data = h.get(name).unwrap();
        let spark_line = Sparkline::default()
            .block(
                Block::new()
                    .borders(Borders::LEFT | Borders::BOTTOM)
                    .title(name),
            )
            .data(data)
            .style(Style::default().fg(Color::Green));
        Widget::render(spark_line, *areas, buf);
    }

    fn render_header(&self, area: Rect, buf: &mut Buffer) {
        let b = Block::new()
            .title(Line::raw(APP_NAME).centered())
            .borders(Borders::ALL);
        let p = Paragraph::new(APP_NAME)
            .style(Style::default().fg(Color::LightCyan))
            .block(b);
        Widget::render(p, area, buf);
    }

    fn render_footer(&self, area: Rect, buf: &mut Buffer) {
        let b = Block::new()
            .title(Line::raw("Updated").centered())
            .borders(Borders::ALL);
        let time: DateTime<Local> = Local::now();
        let p = Paragraph::new(time.format("%H:%M:%S").to_string())
            .style(Style::default().fg(Color::Gray))
            .block(b);
        Widget::render(p, area, buf);
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let h = self.h.lock().unwrap().get_history(HIST_SIZE);
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .areas(area);

        let [dashboard_area, latest_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Percentage(30)]).areas(main_area);

        let line_count = h.keys().count() as u32;
        let mut constraints = vec![];
        for _ in 0..=line_count {
            constraints.push(Constraint::Ratio(1, line_count));
        }
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(dashboard_area);

        self.render_current_values(latest_area, buf);
        let mut index = 0;
        for line_name in h.keys() {
            // if let Some(rect) = chunks[index] {
            self.render_spark_line(&chunks[index], buf, line_name);
            //}
            index += 1;
        }
        self.render_footer(footer_area, buf);
        self.render_header(header_area, buf);
    }
}
