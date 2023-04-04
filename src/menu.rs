use std::fs;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph, Wrap},
    text::{Spans},
    Frame,
};
 
// Menu function draws the main menu
pub fn ui<B: Backend>(f: &mut Frame<B>) {
   let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
            ].as_ref()
        )
        .split(f.size());
    
    let block = Block::default()
         .title("Menu")
         .borders(Borders::ALL);
    f.render_widget(block, chunks[0]);
    
    let text = vec![
        Spans::from("Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat."),
    ];

    let block = Block::default()
         .title("Variable Name")
         .borders(Borders::ALL);
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap {trim:true});
    f.render_widget(paragraph, chunks[1]);
}

pub fn grab_apps() -> Vec<String> {
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

// Just in case I need to check a variable type
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
