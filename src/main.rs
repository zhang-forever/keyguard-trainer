use rand::Rng;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use std::io;

mod app;
mod ui;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = app::App::new();
    let result = app.run(&mut terminal);
    ratatui::restore();
    result
}

/// Falling word entity
#[derive(Clone)]
struct Word {
    text: String,
    x: f64,
    y: f64,
    speed: f64,
    typed: String,
    alive: bool,
}

impl Word {
    fn new(text: String, term_width: u16) -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(2.0..(term_width as f64 - text.len() as f64 - 2.0));
        let speed = rng.gen_range(0.3..1.5);
        Self { text, x, y: 0.0, speed, typed: String::new(), alive: true }
    }
}
