use crossterm::event::{self, Event, KeyCode};
use rand::Rng;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame, Terminal,
};
use std::io;
use std::time::{Duration, Instant};

use crate::Word;

const WORDS: &[&str] = &[
    "hello", "world", "rust", "code", "type", "fast", "system",
    "memory", "async", "trait", "struct", "impl", "match", "loop",
    "cargo", "module", "string", "array", "vector", "result",
];

pub struct App {
    words: Vec<Word>,
    score: u32,
    wpm: f64,
    accuracy: f64,
    total_keystrokes: u32,
    correct_keystrokes: u32,
    lives: u32,
    tick: Instant,
    input: String,
    spawn_timer: Instant,
    game_over: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            words: Vec::new(),
            score: 0,
            wpm: 0.0,
            accuracy: 100.0,
            total_keystrokes: 0,
            correct_keystrokes: 0,
            lives: 10,
            tick: Instant::now(),
            input: String::new(),
            spawn_timer: Instant::now(),
            game_over: false,
        }
    }

    pub fn run(&mut self, terminal: &mut Terminal<impl ratatui::backend::Backend>) -> io::Result<()> {
        loop {
            let term_width = terminal.size()?.width;
            terminal.draw(|f| self.ui(f))?;

            if self.game_over {
                if event::poll(Duration::from_secs(5))? {
                    if let Event::Key(_) = event::read()? {
                        *self = App::new();
                        continue;
                    }
                }
                continue;
            }

            if event::poll(Duration::from_millis(50))? {
                match event::read()? {
                    Event::Key(key) => match key.code {
                        KeyCode::Char(c) => {
                            self.input.push(c);
                            self.total_keystrokes += 1;
                            let mut matched = false;
                            for w in &mut self.words {
                                w.typed.push(c);
                                if w.typed == w.text {
                                    w.alive = false;
                                    self.score += w.text.len() as u32;
                                    self.correct_keystrokes += w.text.len() as u32;
                                    matched = true;
                                } else if !w.text.starts_with(&w.typed) {
                                    w.typed.pop();
                                } else {
                                    matched = true;
                                }
                            }
                            if matched {
                                self.input.clear();
                            } else {
                                self.input.pop();
                            }
                        }
                        KeyCode::Backspace => {
                            self.input.pop();
                            for w in &mut self.words {
                                w.typed.pop();
                            }
                        }
                        KeyCode::Esc => return Ok(()),
                        _ => {}
                    },
                    _ => {}
                }
            }

            self.update(term_width);
        }
    }

    fn update(&mut self, term_width: u16) {
        let mut rng = rand::thread_rng();

        if self.spawn_timer.elapsed() > Duration::from_millis(2000) {
            let word_text = WORDS[rng.gen_range(0..WORDS.len())].to_string();
            self.words.push(Word::new(word_text, term_width));
            self.spawn_timer = Instant::now();
        }

        let dt = self.tick.elapsed().as_secs_f64();
        self.tick = Instant::now();

        for w in &mut self.words {
            w.y += w.speed * dt * 5.0;
        }

        self.words.retain(|w| {
            if !w.alive {
                return false;
            }
            if w.y > 20.0 {
                self.lives = self.lives.saturating_sub(1);
                if self.lives == 0 {
                    self.game_over = true;
                }
                return false;
            }
            true
        });

        let elapsed = Instant::now().duration_since(Instant::now()).as_secs_f64().max(0.001);
        self.wpm = (self.correct_keystrokes as f64 / 5.0) / (elapsed / 60.0);
        if self.total_keystrokes > 0 {
            self.accuracy = (self.correct_keystrokes as f64 / self.total_keystrokes as f64) * 100.0;
        }
    }

    fn ui(&self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(3),
            ])
            .split(f.area());

        let stats = format!(
            "Score: {} | WPM: {:.0} | Accuracy: {:.1}% | Lives: {}",
            self.score, self.wpm, self.accuracy, self.lives
        );
        f.render_widget(
            Paragraph::new(stats)
                .block(Block::default().borders(Borders::ALL).title("KEYGUARD TRAINER")),
            chunks[0],
        );

        if self.game_over {
            f.render_widget(
                Paragraph::new("GAME OVER — Press any key to restart")
                    .style(Style::default().fg(Color::Red))
                    .block(Block::default().borders(Borders::ALL)),
                chunks[1],
            );
        } else {
            let words_text: Vec<ratatui::text::Line> = self
                .words
                .iter()
                .map(|w| {
                    let y_pad = " ".repeat(w.y as usize);
                    let typed_len = w.typed.len();
                    let correct = &w.text[..typed_len.min(w.text.len())];
                    let remaining = &w.text[typed_len.min(w.text.len())..];
                    let line = format!(
                        "{}{}{}{}",
                        " ".repeat(w.x as usize),
                        y_pad,
                        correct,
                        remaining
                    );
                    ratatui::text::Line::from(line)
                })
                .collect();

            f.render_widget(
                Paragraph::new(Text::from(words_text))
                    .block(Block::default().borders(Borders::ALL)),
                chunks[1],
            );
        }

        f.render_widget(
            Paragraph::new(format!("> {}", self.input))
                .style(Style::default().fg(Color::Green)),
            chunks[2],
        );
    }
}
