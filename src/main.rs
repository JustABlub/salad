use std::{io, thread, time::Duration};
use keycodes::keyCode;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    widgets::{Widget, Block, Borders},
    layout::{Layout, Constraint, Direction},
    Terminal
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
mod menu;
mod grab_apps;

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, menu::Menu_Bar);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    grab_apps::grab_apps();

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut menu_bar: menu::MenuBar) -> io::Result<()> {
    loop {
        terminal.draw(|f| menu::ui(f, &mut app))?;

        //TODO see if I can actually quit using q
        if let Event::Key(key) = event::read()? {
            match key.code {
                keyCode::Char('q') => return Ok(()),
                KeyCode::Up => menu_bar.previous(),
                keyCode::Down => menu_bar.next(),
                _ => {}
            }
        }
    }
}
