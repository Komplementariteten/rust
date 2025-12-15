use crate::store::StoreItem;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Rect};
use ratatui::prelude::*;
use ratatui::style::palette::tailwind::SLATE;
use ratatui::style::Style;
use ratatui::text::{Line, Text, ToSpan};
use ratatui::widgets::{Block, Borders, Chart, HighlightSpacing, List, ListItem, ListState, Sparkline};
use ratatui::{layout, layout::Layout, DefaultTerminal, Frame};
use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::time::Duration;
use crate::history::History;

struct UiState {
    latest_update: HashMap<String, u32>,
    history: HashMap<String, Vec<u64>>,
}

impl UiState {
    pub fn new(h: &History) -> UiState {
        UiState {
            latest_update: h.individual_values.clone(),
            history: h.get_history(),
        }
    }
}

pub struct App {
    state: UiState,
    current_value_state: ListState,
}

impl App {
    fn new(h: &History) -> App {
        App {
            state: UiState::new(h),
            current_value_state: ListState::default(),
        }
    }

    fn update_state(&mut self, v: StoreItem) {
        self.state
            .latest_update
            .entry(v.item_name)
            .and_modify(|x| *x = v.value)
            .or_insert(v.value);
    }

    fn draw(&mut self, f: &mut Frame) {
        f.render_widget(self, f.area())
    }

    pub(crate) fn run(mut terminal: DefaultTerminal, rx: Receiver<StoreItem>, h: &History) {
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
            if let Ok(ok) = event::poll(Duration::from_millis(10)) && ok {
                match event::read().expect("event failed") {
                    Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                        KeyCode::Esc => return,
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
        let data = self.state.history.get(name).cloned().unwrap_or_default();
        let spark_line = Sparkline::default().block(Block::new().borders(Borders::LEFT | Borders::BOTTOM).title(name)).data(&data).style(Style::default().fg(Color::Green));
        Widget::render(spark_line, *areas, buf);
    }

    fn render_header(&self, area: Rect, buf: &mut Buffer) {

    }

    fn render_footer() {}
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let [header_area, main_area, footer_area] = layout::Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .areas(area);

        let [dashboard_area, latest_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Percentage(30)]).areas(main_area);

        let line_count = self.state.history.keys().count() as u32;
        let mut constraints = vec![];
        for i in 0..=line_count {
            constraints.push(Constraint::Ratio(1, line_count));
        }
        let chunks = Layout::default().direction(Direction::Vertical).constraints(constraints).split(dashboard_area);

        self.render_current_values(latest_area, buf);
        for line_name in self.state.history.clone().keys() {
             if let Some(rect) = chunks.iter().next() {
                 self.render_spark_line(rect, buf, line_name);
             }
        }
    }
}
