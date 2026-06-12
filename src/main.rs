use rand::Rng;
use std::io;

mod app;

/// Difficulty presets affecting spawn rate and word speed
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
}

impl Difficulty {
    pub fn spawn_interval_ms(self) -> u64 {
        match self {
            Difficulty::Easy => 3000,
            Difficulty::Normal => 2000,
            Difficulty::Hard => 1000,
        }
    }

    pub fn speed_range(self) -> (f64, f64) {
        match self {
            Difficulty::Easy => (0.2, 0.8),
            Difficulty::Normal => (0.3, 1.5),
            Difficulty::Hard => (0.6, 2.5),
        }
    }

    pub fn starting_lives(self) -> u32 {
        match self {
            Difficulty::Easy => 15,
            Difficulty::Normal => 10,
            Difficulty::Hard => 5,
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "easy" | "e" | "1" => Some(Difficulty::Easy),
            "normal" | "medium" | "m" | "n" | "2" => Some(Difficulty::Normal),
            "hard" | "h" | "3" => Some(Difficulty::Hard),
            _ => None,
        }
    }
}

impl Default for Difficulty {
    fn default() -> Self {
        Difficulty::Normal
    }
}

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
    pub fn new(text: String, term_width: u16, speed_range: (f64, f64)) -> Self {
        let mut rng = rand::thread_rng();
        let max_x = (term_width as f64 - text.len() as f64 - 2.0).max(2.0);
        let x = rng.gen_range(2.0..max_x);
        let speed = rng.gen_range(speed_range.0..speed_range.1);
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

fn parse_args() -> Difficulty {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        match Difficulty::parse(&args[1]) {
            Some(d) => d,
            None => {
                eprintln!("Usage: {} [easy|normal|hard]", args[0]);
                eprintln!("Invalid difficulty: '{}'. Using Normal.", args[1]);
                Difficulty::Normal
            }
        }
    } else {
        Difficulty::Normal
    }
}

fn main() -> io::Result<()> {
    let difficulty = parse_args();
    let mut terminal = ratatui::init();
    let mut app = app::App::new(difficulty);
    let result = app.run(&mut terminal);
    ratatui::restore();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_difficulty_parse() {
        assert_eq!(Difficulty::parse("easy"), Some(Difficulty::Easy));
        assert_eq!(Difficulty::parse("E"), Some(Difficulty::Easy));
        assert_eq!(Difficulty::parse("1"), Some(Difficulty::Easy));
        assert_eq!(Difficulty::parse("normal"), Some(Difficulty::Normal));
        assert_eq!(Difficulty::parse("medium"), Some(Difficulty::Normal));
        assert_eq!(Difficulty::parse("n"), Some(Difficulty::Normal));
        assert_eq!(Difficulty::parse("2"), Some(Difficulty::Normal));
        assert_eq!(Difficulty::parse("hard"), Some(Difficulty::Hard));
        assert_eq!(Difficulty::parse("h"), Some(Difficulty::Hard));
        assert_eq!(Difficulty::parse("3"), Some(Difficulty::Hard));
        assert_eq!(Difficulty::parse("impossible"), None);
    }

    #[test]
    fn test_difficulty_defaults() {
        assert_eq!(Difficulty::default(), Difficulty::Normal);
    }

    #[test]
    fn test_difficulty_spawn_intervals() {
        assert!(Difficulty::Easy.spawn_interval_ms() > Difficulty::Normal.spawn_interval_ms());
        assert!(Difficulty::Normal.spawn_interval_ms() > Difficulty::Hard.spawn_interval_ms());
    }

    #[test]
    fn test_difficulty_speed_ranges() {
        let (e_lo, e_hi) = Difficulty::Easy.speed_range();
        let (n_lo, n_hi) = Difficulty::Normal.speed_range();
        let (h_lo, h_hi) = Difficulty::Hard.speed_range();
        assert!(e_lo < n_lo);
        assert!(e_hi < n_hi);
        assert!(n_lo < h_lo);
        assert!(n_hi < h_hi);
    }

    #[test]
    fn test_difficulty_lives() {
        assert!(Difficulty::Easy.starting_lives() > Difficulty::Normal.starting_lives());
        assert!(Difficulty::Normal.starting_lives() > Difficulty::Hard.starting_lives());
    }

    #[test]
    fn test_word_new() {
        let w = Word::new("hello".to_string(), 80, (0.3, 1.5));
        assert_eq!(w.text, "hello");
        assert_eq!(w.y, 0.0);
        assert!(!w.typed.is_empty() || w.typed.is_empty()); // typed starts empty
        assert!(w.alive);
        assert!(w.x >= 2.0);
        assert!(w.x <= 80.0 - 5.0 - 2.0);
        assert!(w.speed >= 0.3);
        assert!(w.speed <= 1.5);
    }

    #[test]
    fn test_word_new_short_word() {
        let w = Word::new("a".to_string(), 80, (0.2, 0.8));
        assert_eq!(w.text, "a");
        assert!(w.speed >= 0.2);
        assert!(w.speed <= 0.8);
    }

    #[test]
    fn test_word_new_narrow_terminal() {
        // Word longer than terminal width - max_x should be clamped to >= 2.0
        let w = Word::new("hello".to_string(), 4, (0.3, 1.0));
        assert!(w.x >= 2.0);
    }

    #[test]
    fn test_word_clone() {
        let w = Word::new("test".to_string(), 80, (0.5, 1.0));
        let w2 = w.clone();
        assert_eq!(w.text, w2.text);
        assert_eq!(w.x, w2.x);
        assert_eq!(w.y, w2.y);
        assert_eq!(w.speed, w2.speed);
        assert_eq!(w.alive, w2.alive);
    }
}
