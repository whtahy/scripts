use std::io::Write;

// Rules
// Game has 10 frames.
// Each frame starts with 10 pins.
// Max 2 rolls per frame.
// Last frame only: spare OR strike -> exactly 3 rolls.
const LAST_FRAME: usize = 9;
const MAX_PINS: T = 10;
const FRAME_SIZE: usize = 2;
const FRAME_SIZE_LAST: usize = FRAME_SIZE + 1;

type T = u32;

struct Game {
    frames: Vec<Vec<T>>,
}

use CompletionStatus::*;
#[derive(PartialEq)]
enum CompletionStatus {
    NoBonus,
    Strike,
    Spare,
    Incomplete,
}

impl Game {
    fn new() -> Game {
        Game {
            frames: vec![vec![]],
        }
    }

    fn print(&self) {
        println!("Frame={}, Score={}", self.frames.len(), self.total_score());
        println!("Frames: {:?}", self.frames);
        println!("Scores: {:?}", self.scores());
    }

    fn roll(&mut self, p: &T) -> Result<(), &str> {
        self.valid(p)?;
        let i = self.frames.len() - 1;
        self.frames[i].push(*p);
        if i < LAST_FRAME && self.completion_status(i) != Incomplete {
            self.frames.push(vec![]);
        }
        Ok(())
    }

    fn score(&self, i: usize) -> Option<T> {
        self.frames.get(i)?;
        let sum = self.sum(i);

        match self.completion_status(i) {
            Incomplete => None,
            Strike => {
                let n = self.frames.iter().take(i + 1).flatten().count();
                let mut lookahead = self.frames.iter().flatten().skip(n);
                Some(MAX_PINS + lookahead.next()? + lookahead.next()?)
            }
            Spare => Some(MAX_PINS + self.frames.get(i + 1)?.get(0)?),
            NoBonus => Some(sum),
        }
    }

    fn scores(&self) -> Vec<T> {
        (0..self.frames.len()).flat_map(|i| self.score(i)).collect()
    }

    fn valid<'a>(&self, p: &T) -> Result<(), &'a str> {
        let i = self.frames.len() - 1;
        let remaining_pins = if self.sum(i) % MAX_PINS == 0 {
            MAX_PINS
        } else {
            MAX_PINS - self.frames[i].last().unwrap()
        };

        if i == LAST_FRAME && self.completion_status(i) != Incomplete {
            Err("Game Over!")
        } else if p > &remaining_pins {
            Err("Invalid # of pins!")
        } else {
            Ok(())
        }
    }

    fn completion_status(&self, i: usize) -> CompletionStatus {
        let len = self.frames[i].len();
        let sum = self.sum(i);

        if len == FRAME_SIZE_LAST || (len == FRAME_SIZE && sum < MAX_PINS) {
            NoBonus
        } else if i < LAST_FRAME && len == 1 && sum == MAX_PINS {
            Strike
        } else if i < LAST_FRAME && len == FRAME_SIZE && sum == MAX_PINS {
            Spare
        } else {
            Incomplete
        }
    }

    fn sum(&self, i: usize) -> T {
        self.frames[i].iter().sum()
    }

    fn total_score(&self) -> T {
        self.scores().iter().sum()
    }

    fn undo(&mut self) -> Option<()> {
        if self.frames[0].is_empty() {
            None
        } else {
            if self.frames.last()?.is_empty() {
                self.frames.pop();
            }
            self.frames.last_mut()?.pop();
            Some(())
        }
    }
}

fn main() {
    std::process::Command::new("clear").status().unwrap();

    let mut game = Game::new();
    game.print();

    loop {
        print!(" > ");
        std::io::stdout().flush().unwrap();

        let input = user_input();
        match input.as_str() {
            "exit" => break,
            "restart" => game = Game::new(),
            "undo" => {
                game.undo();
            }
            "clear" => {
                std::process::Command::new("clear").status().unwrap();
            }
            _ => {
                if let Ok(x) = input.parse() {
                    if let Err(s) = game.roll(&x) {
                        println!("{}", s);
                    }
                }
            }
        }
        game.print();
    }
}

fn user_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn mock(rolls: &[T]) -> Game {
        let mut g = Game::new();
        for r in rolls {
            let _ = g.roll(r);
        }
        g
    }

    fn score(rolls: &[T]) -> T {
        mock(rolls).total_score()
    }

    fn mock_str(rolls: &str) -> Game {
        let mut g = Game::new();
        for s in rolls.split_whitespace() {
            let r = s.parse().unwrap();
            let _ = g.roll(&r);
        }
        g
    }

    fn score_str(rolls: &str) -> T {
        mock_str(rolls).total_score()
    }

    #[test]
    fn test_streaks() {
        assert_eq!(0, score(&[0; 20 + 1]));
        assert_eq!(20, score(&[1; 20 + 1]));
        assert_eq!(40, score(&[2; 20 + 1]));
        assert_eq!(60, score(&[3; 20 + 1]));
        assert_eq!(80, score(&[4; 20 + 1]));
        assert_eq!(150, score(&[5; 21 + 1]));
        assert_eq!(300, score(&[10; 12 + 1]));
    }

    #[test]
    fn test_open_frames() {
        // frame 1     2     3     4     5     6     7     8     9     10
        let a = "4  5  5  4  5  4  5  4  5  4  5  4  5  4  5  4  5  4  5  4";
        let b = "1  2  3  4  5  4  3  2  1  0  9  0  0  9  9  0  0  1  0  0";
        let c = "0  2  0  2  0  2  0  2  0  2  0  2  0  2  0  2  0  2  0  2";
        let d = "3  0  3  0  3  0  3  0  3  0  3  0  3  0  3  0  3  0  3  0";
        assert_eq!(90, score_str(a));
        assert_eq!(53, score_str(b));
        assert_eq!(20, score_str(c));
        assert_eq!(30, score_str(d));
    }

    #[test]
    fn test_spares() {
        // frame 1     2     3     4     5     6     7     8     9     10
        let a = "6  4  5  5  6  4  7  3  1  9  2  8  6  4  5  5  7  3  6  4  4";
        assert_eq!(149, score_str(a));
    }

    #[test]
    fn test_strikes() {
        assert_eq!(25, score(&[10, 5, 0, 5, 0]));
        assert_eq!(31, score(&[10, 5, 2, 5, 2]));
        assert_eq!(23, score(&[5, 0, 10, 2, 2]));
        assert_eq!(40, score(&[10, 10, 2, 2]));
        assert_eq!(70, score(&[10, 10, 10, 2, 2]));
        assert_eq!(100, score(&[10, 10, 10, 10, 2, 2]));
    }

    #[test]
    fn test_spares_and_strikes() {
        // strike before spare
        assert_eq!(32, score(&[10, 5, 5, 2]));
        assert_eq!(36, score(&[10, 5, 5, 2, 2]));
        assert_eq!(58, score(&[10, 5, 5, 10, 2, 2]));
        assert_eq!(60, score(&[10, 5, 5, 10, 5, 5]));
        assert_eq!(72, score(&[10, 5, 5, 10, 5, 5, 2]));
        assert_eq!(61, score(&[10, 10, 5, 10, 5, 2, 2]));

        // spare before strike
        assert_eq!(40, score(&[5, 5, 10, 5, 5]));
        assert_eq!(52, score(&[5, 5, 10, 5, 5, 2]));
    }

    #[test]
    fn test_last_frame() {
        fn f(frame: Vec<T>) -> Vec<T> {
            std::iter::repeat(0).take(9 * 2).chain(frame).collect()
        }

        // last frame score = sum(frame)
        assert_eq!(20, score(&f(vec![10, 6, 4])));
        assert_eq!(24, score(&f(vec![10, 10, 4])));
        assert_eq!(15, score(&f(vec![5, 5, 5])));
        assert_eq!(15, score(&f(vec![1, 9, 5])));
        assert_eq!(15, score(&f(vec![10, 0, 5])));
        assert_eq!(20, score(&f(vec![5, 5, 10])));

        // extra rolls after spare/strike
        assert_eq!(20, score(&f(vec![10, 6, 4, 1])));
        assert_eq!(24, score(&f(vec![10, 10, 4, 1])));
        assert_eq!(15, score(&f(vec![5, 5, 5, 1])));
        assert_eq!(15, score(&f(vec![1, 9, 5, 1])));
        assert_eq!(15, score(&f(vec![10, 0, 5, 1])));
        assert_eq!(20, score(&f(vec![5, 5, 10, 1])));

        // extra rolls after open frame
        assert_eq!(9, score(&f(vec![4, 5, 5, 1])));
        assert_eq!(5, score(&f(vec![5, 0, 5, 1])));
        assert_eq!(0, score(&f(vec![0, 0, 5, 1])));

        // invalid inputs
        assert_eq!(1, score(&f(vec![1, 10, 0, 1, 1])));
        assert_eq!(2, score(&f(vec![1, 10, 1, 1, 1])));
        assert_eq!(11, score(&f(vec![5, 10, 5, 1, 1, 1])));
    }

    #[test]
    fn test_undo() {
        fn f(p: T, rolls: impl Iterator<Item = T>) {
            let mut a = Game::new();
            let mut b = Game::new();
            for r in rolls {
                let _ = a.roll(&r);
                let _ = b.roll(&r);
            }
            if b.roll(&p).is_ok() {
                b.undo();
            };
            assert_eq!(a.total_score(), b.total_score());
        }

        for p in 0..=11 {
            for x in 0..=10 {
                for y in 0..=30 {
                    f(p, std::iter::repeat(x).take(y));
                }
            }
        }
    }
}
