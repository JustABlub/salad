use std::{io, thread, time::Duration};
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

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    //terminal.draw(|f| {
    //    let size = f.size();
    //    let block = Block::default()
    //        .title("Salad")
    //        .borders(Borders::ALL);
    //    f.render_widget(block, size);
    //})?;
    let res = run_app(&mut terminal);

    thread::sleep(Duration::from_millis(5000));

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(|f| menu::ui(f));

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(())
            }
        }
    }
}
