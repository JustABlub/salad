use std::{fs, error::Error, io};
use super::grab_apps;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph, Wrap, Cell, Row, Table, TableState},
    text::{Spans},
    Frame,
};
 
pub struct MenuBar<'a> {
    state: TableState,
    items: Vec<Vec<&'a str>>,
}

impl<'a> MenuBar <'a> {
    fn new() -> MenuBar<'a> {
        MenuBar {
            state: TableState::default(),
            items: vec![grab_apps::grab_apps()],
        }
    }
    
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                }
                else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.selected(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len()
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

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
        Spans::from("Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.")
    ];

    let block = Block::default()
         .title("Variable Name")
         .borders(Borders::ALL);
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap {trim:true});
    f.render_widget(paragraph, chunks[1]);
}


// Just in case I need to check a variable type
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
