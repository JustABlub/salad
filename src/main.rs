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

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    grab_apps();

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(|f| menu::ui(f));

        //TODO see if I can actually quit using q
        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(())
            }
        }
    }
}
fn grab_apps() -> Vec<String> {
    // Read ./apps/ directory and creates a vector with all files in it
    let paths = fs::read_dir("/home/blub/Projects/rust/salad/src/apps/")
       .unwrap()
       .filter_map(|e| e.ok())
       .map(|e| e.path())
       .collect::<Vec<_>>();
    
    // Filter out and clean up vector so we can return just the app names
    let mut app_vec = Vec::new();
    for i in &paths {
        app_vec.push(i.clone().into_os_string().into_string().unwrap());
    }
    println!("{:?}", &app_vec);
    for i in &mut app_vec {
        let stripped = i.split('/').last().unwrap().to_string();
        *i = stripped.split('.').next().unwrap().to_string();
    }
    // debug statement 
    println!("{:?}", &app_vec);
    return app_vec;
}

