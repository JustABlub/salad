use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph, Wrap},
    text::{Spans},
    Frame,
};
 

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
