use crate::app::{App, InputContext};
use std::borrow::Borrow;
use std::fs::{read_dir, DirEntry, ReadDir};
use std::path::{Path, PathBuf};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans, Text};

pub fn get_files_in_path(app: App) {}

pub fn list_files(path: PathBuf) -> Vec<String> {
    let mut vec = Vec::new();

    if let Ok(dir_content) = read_dir(path) {
        for dir_entry in dir_content {
            if dir_entry.is_ok() && !dir_entry?.path().is_dir() {
                let file = dir_entry.unwrap();
                vec.push(file.path().file_name()?.to_str()?.to_string());
            }
        }
    }
    return vec;
}

pub fn get_menu<'a>(app: &'a mut App) -> Spans<'a> {
    return Spans::from(vec![
        Span::styled(
            "[F4] add",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("  |  ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            "[F4] add file",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("  |  ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            "[F4] p",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
    ]);
}

pub fn complete_folder<'a>(app: &'a mut App) -> Spans<'a> {
    let base_p = Path::new(&app.ds_base_path);

    if app.entered_text.len() == 0 || app.context != InputContext::DataStorePath {
        return Spans::from(vec![Span::styled(
            &app.ds_base_path,
            Style::default().fg(Color::Black).bg(Color::Gray),
        )]);
    }
    let entered_path = base_p.join(Path::new(&app.entered_text));
    if entered_path.exists() {
        let mut string = app.ds_base_path.to_owned();
        string.push_str(app.entered_text.as_str());
        string.push('/');
        app.ds_base_path = string;
        app.entered_text = String::new();
        return Spans::from(vec![Span::styled(
            &app.ds_base_path,
            Style::default().fg(Color::Black).bg(Color::Gray),
        )]);
    }

    if let Ok(dir_content) = read_dir(base_p) {
        let matches = dir_content.filter(|d| d.is_ok() && d.as_ref().unwrap().path().is_dir());
        for x in matches {
            let str_box = Box::new(x.unwrap().file_name().to_str().unwrap().to_string());
            if str_box.starts_with(&app.entered_text) {
                if app.should_tab {
                    let mut string = app.ds_base_path.to_owned();
                    string.push_str(str_box.as_str());
                    string.push('/');
                    app.ds_base_path = string;
                    app.entered_text = String::new();
                    app.should_tab = false;
                    return Spans::from(vec![Span::styled(
                        &app.ds_base_path,
                        Style::default().fg(Color::Black).bg(Color::Gray),
                    )]);
                }

                return Spans::from(vec![
                    Span::styled(
                        &app.ds_base_path,
                        Style::default().fg(Color::Black).bg(Color::Gray),
                    ),
                    Span::styled(
                        &app.entered_text,
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        str_box.strip_prefix(&app.entered_text).unwrap().to_string(),
                        Style::default().fg(Color::DarkGray),
                    ),
                ]);
            }
        }
    }
    return Spans::from(vec![
        Span::styled(
            &app.ds_base_path,
            Style::default().fg(Color::White).bg(Color::Gray),
        ),
        Span::styled(&app.entered_text, Style::default().fg(Color::Red)),
    ]);
}
