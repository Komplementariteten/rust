use crate::app::{App, InputContext};
use crate::ui;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{event, execute};
use std::error::Error;
use std::io;
use std::time::{Duration, Instant};
use tui::backend::{Backend, CrosstermBackend};
use tui::Terminal;

pub fn run(tick_rate: Duration, enhanced_graphics: bool) -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new("datastore", enhanced_graphics);
    let res = run_app(&mut terminal, app, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::F(1) => app.activate_one(),
                    KeyCode::F(2) => app.activate_two(),
                    KeyCode::F(3) => app.activate_three(),
                    KeyCode::F(4) => app.should_f4 = true,
                    KeyCode::F(5) => app.should_f5 = true,
                    KeyCode::F(6) => app.should_f6 = true,
                    KeyCode::F(7) => app.should_f7 = true,
                    KeyCode::F(8) => app.should_f8 = true,
                    KeyCode::F(9) => app.should_f9 = true,
                    KeyCode::Esc => app.quit(),
                    KeyCode::Tab => app.tab(),
                    KeyCode::Enter => app.submit(),
                    KeyCode::Delete => app.delete(),
                    KeyCode::Backspace => app.delete(),
                    KeyCode::Char(c) => app.on_key(c),
                    /* KeyCode::Char(c) => app.on_key(c),
                    KeyCode::Left => app.on_left(),
                    KeyCode::Up => app.on_up(),
                    KeyCode::Right => app.on_right(),
                    KeyCode::Down => app.on_down(),
                    KeyCode::Tab => app.next_app(), */
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
        if app.should_quit {
            return Ok(());
        }
    }
}
