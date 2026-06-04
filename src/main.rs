use rand::Rng;
use std::io;

mod app;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = app::App::new();
    let result = app.run(&mut terminal);
    ratatui::restore();
    result
}

/// Falling word entity
#[derive(Clone)]
pub struct Word {
    pub text: String,
    pub x: f64,
    pub y: f64,
    pub speed: f64,
    pub typed: String,
    pub alive: bool,
}

impl Word {
    pub fn new(text: String, term_width: u16) -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(2.0..(term_width as f64 - text.len() as f64 - 2.0));
        let speed = rng.gen_range(0.3..1.5);
        Self {
            text,
            x,
            y: 0.0,
            speed,
            typed: String::new(),
            alive: true,
        }
    }
}
