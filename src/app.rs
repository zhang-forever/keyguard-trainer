use crossterm::event::{self, Event, KeyCode};
use rand::Rng;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::io;
use std::time::{Duration, Instant};

use crate::{Difficulty, Word};

const WORDS: &[&str] = &[
    "hello", "world", "rust", "code", "type", "fast", "system", "memory", "async", "trait",
    "struct", "impl", "match", "loop", "cargo", "module", "string", "array", "vector", "result",
    "option", "iter", "macro", "slice", "clone", "debug", "error", "stack", "queue", "hash",
    "tree", "sort", "filter", "reduce", "panic", "mutex", "thread", "spawn", "future", "tokio",
    "unsafe", "ref", "move", "pub", "self", "super", "crate", "where", "enum", "box",
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
    difficulty: Difficulty,
    game_start: Instant,
    words_typed: u32,
}

impl App {
    pub fn new(difficulty: Difficulty) -> Self {
        Self {
            words: Vec::new(),
            score: 0,
            wpm: 0.0,
            accuracy: 100.0,
            total_keystrokes: 0,
            correct_keystrokes: 0,
            lives: difficulty.starting_lives(),
            tick: Instant::now(),
            input: String::new(),
            spawn_timer: Instant::now(),
            game_over: false,
            difficulty,
            game_start: Instant::now(),
            words_typed: 0,
        }
    }

    pub fn run(
        &mut self,
        terminal: &mut Terminal<impl ratatui::backend::Backend>,
    ) -> io::Result<()> {
        loop {
            let term_width = terminal.size()?.width;
            terminal.draw(|f| self.ui(f))?;

            if self.game_over {
                if event::poll(Duration::from_secs(5))? {
                    if let Event::Key(_) = event::read()? {
                        *self = App::new(self.difficulty);
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
                                if !w.alive {
                                    continue;
                                }
                                w.typed.push(c);
                                if w.typed == w.text {
                                    w.alive = false;
                                    self.score += w.text.len() as u32;
                                    self.correct_keystrokes += w.text.len() as u32;
                                    self.words_typed += 1;
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
                                if w.alive {
                                    w.typed.pop();
                                }
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

        if self.spawn_timer.elapsed() > Duration::from_millis(self.difficulty.spawn_interval_ms()) {
            let word_text = WORDS[rng.gen_range(0..WORDS.len())].to_string();
            self.words.push(Word::new(
                word_text,
                term_width,
                self.difficulty.speed_range(),
            ));
            self.spawn_timer = Instant::now();
        }

        let dt = self.tick.elapsed().as_secs_f64();
        self.tick = Instant::now();

        for w in &mut self.words {
            if w.alive {
                w.y += w.speed * dt * 5.0;
            }
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

        // WPM: based on characters typed / 5 / minutes elapsed
        let elapsed = self.game_start.elapsed().as_secs_f64().max(1.0);
        let minutes = elapsed / 60.0;
        if minutes > 0.0 {
            self.wpm = (self.correct_keystrokes as f64 / 5.0) / minutes;
        }
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

        let diff_label = match self.difficulty {
            Difficulty::Easy => "Easy",
            Difficulty::Normal => "Normal",
            Difficulty::Hard => "Hard",
        };
        let stats = format!(
            "Score: {} | WPM: {:.0} | Accuracy: {:.1}% | Lives: {} | Difficulty: {}",
            self.score, self.wpm, self.accuracy, self.lives, diff_label
        );
        f.render_widget(
            Paragraph::new(stats).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("KEYGUARD TRAINER"),
            ),
            chunks[0],
        );

        if self.game_over {
            let summary = format!(
                "GAME OVER — Score: {} | WPM: {:.0} | Words: {} | Press any key to restart",
                self.score, self.wpm, self.words_typed
            );
            f.render_widget(
                Paragraph::new(summary)
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
                    let typed_len = w.typed.len().min(w.text.len());
                    let correct = &w.text[..typed_len];
                    let remaining = &w.text[typed_len..];
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
            Paragraph::new(format!("> {}", self.input)).style(Style::default().fg(Color::Green)),
            chunks[2],
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_new_default_difficulty() {
        let app = App::new(Difficulty::Normal);
        assert_eq!(app.score, 0);
        assert_eq!(app.lives, 10);
        assert_eq!(app.accuracy, 100.0);
        assert!(!app.game_over);
        assert!(app.words.is_empty());
        assert!(app.input.is_empty());
        assert_eq!(app.correct_keystrokes, 0);
        assert_eq!(app.total_keystrokes, 0);
    }

    #[test]
    fn test_app_new_easy_difficulty() {
        let app = App::new(Difficulty::Easy);
        assert_eq!(app.lives, 15);
    }

    #[test]
    fn test_app_new_hard_difficulty() {
        let app = App::new(Difficulty::Hard);
        assert_eq!(app.lives, 5);
    }

    #[test]
    fn test_app_difficulty_preserved() {
        let app = App::new(Difficulty::Hard);
        assert_eq!(app.difficulty, Difficulty::Hard);
    }

    #[test]
    fn test_word_list_not_empty() {
        assert!(!WORDS.is_empty());
        for w in WORDS {
            assert!(!w.is_empty(), "Word list contains empty string");
        }
    }

    #[test]
    fn test_word_list_no_duplicates() {
        let mut sorted = WORDS.to_vec();
        sorted.sort();
        let before = sorted.len();
        sorted.dedup();
        assert_eq!(before, sorted.len(), "Word list has duplicates");
    }
}
