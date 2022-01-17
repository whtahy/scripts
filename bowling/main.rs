use std::io::Write;
use CompletionStatus::*;

// Rules
// Game has 10 frames.
// Each frame starts with 10 pins.
// Max 2 rolls per frame.
// Last frame only: spare OR strike -> exactly 3 rolls.
const LAST_FRAME: usize = 9;
const MAX_PINS: T = 10;
const FRAME_SIZE: usize = 2;
const FRAME_SIZE_LAST: usize = FRAME_SIZE + 1;

#[derive(PartialEq)]
enum CompletionStatus {
    NoBonus,
    Strike,
    Spare,
    Incomplete,
}

type T = u32;

struct Game {
    frames: Vec<Vec<T>>,
}

impl Game {
    fn new() -> Game {
        Game {
            frames: vec![vec![]],
        }
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

    fn scores(&self) -> impl Iterator<Item = T> + '_ {
        (0..self.frames.len()).flat_map(|i| self.score(i))
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

    fn print(&self) {
        for i in 0..=LAST_FRAME {
            if i < LAST_FRAME {
                print!("{}:[{:3}]  ", i + 1, self.frame_to_string(i));
            } else {
                println!("{}:[{:5}]", i + 1, self.frame_to_string(i));
            }
        }
        for i in 0..=LAST_FRAME {
            let score = if self.score(i).is_some() {
                self.scores().take(i + 1).sum::<T>().to_string()
            } else {
                "".to_string()
            };
            if i < LAST_FRAME {
                print!("  [{:>3}]  ", score);
            } else {
                println!("   [{:>5}]", score);
            }
        }
    }

    fn frame_to_string(&self, i: usize) -> String {
        if i >= self.frames.len() || self.frames[i].is_empty() {
            return "".to_string();
        }

        let f = &self.frames[i];
        let parse = |i: usize| {
            if f[i] == 10 {
                "X".to_string()
            } else {
                f[i].to_string()
            }
        };
        let sum = if f.len() < FRAME_SIZE_LAST {
            self.sum(i)
        } else {
            f[0] + f[1]
        };

        if i == LAST_FRAME {
            match (f.len(), f[0], sum) {
                (3, MAX_PINS, _) => format!("X,{},{}", parse(1), parse(2)),
                (2, MAX_PINS, _) => format!("X,{},", parse(1)),
                (1, MAX_PINS, _) => "X,".to_string(),
                (3, _, MAX_PINS) => format!("{},/,{}", parse(0), parse(2)),
                (2, _, MAX_PINS) => format!("{},/,", parse(0)),
                (3, _, _) => format!("{},{},{}", parse(0), parse(1), parse(2)),
                (2, _, _) => format!("{},{}", parse(0), parse(1)),
                (1, _, _) => format!("{},", parse(0)),
                _ => unreachable!(),
            }
        } else {
            match (f.len(), sum) {
                (1, MAX_PINS) => " X ".to_string(),
                (1, _) => format!("{},", parse(0)),
                (2, MAX_PINS) => format!("{},/", parse(0)),
                (2, _) => format!("{},{}", parse(0), parse(1)),
                _ => unreachable!(),
            }
        }
    }
}

fn main() {
    std::process::Command::new("clear").status().unwrap();
    let mut game = Game::new();
    let mut refresh = true;

    loop {
        if refresh {
            game.print();
        }
        print!(" > ");
        std::io::stdout().flush().unwrap();

        let input = user_input();
        match input.as_str() {
            "exit" => {
                clear();
                break;
            }
            "restart" => {
                game = Game::new();
                clear();
                refresh = true;
            }
            "undo" => {
                game.undo();
                println!();
                refresh = true;
            }
            "clear" => {
                clear();
                refresh = true;
            }
            _ => {
                let err = match input.parse().map(|x| game.roll(&x)) {
                    Err(_) => "Invalid input!",
                    Ok(Err(s)) => s,
                    _ => "",
                };
                println!("{err}");
                refresh = err.is_empty();
            }
        }
    }
}

fn user_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

fn clear() {
    std::process::Command::new("clear").status().unwrap();
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
        mock(rolls).scores().sum()
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
        mock_str(rolls).scores().sum()
    }

    fn pad(last_frame: &[T]) -> Game {
        let mut g = mock(&[0; LAST_FRAME * 2]);
        for p in last_frame {
            let _ = g.roll(p);
        }
        g
    }

    // #[test]
    // fn test_txt() {
    //     let input = include_str!("./input.txt");
    //     let ans = input
    //         .lines()
    //         .map(score_str)
    //         .map(|x| x.to_string() + "\r\n")
    //         .collect::<String>();
    //     assert!(std::fs::write("./ans.txt", ans).is_ok());
    // }

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
        let f = |x: &[T]| pad(x).scores().sum::<T>();

        // last frame score = sum(frame)
        assert_eq!(20, f(&[10, 6, 4]));
        assert_eq!(15, f(&[5, 5, 5]));
        assert_eq!(15, f(&[1, 9, 5]));
        assert_eq!(15, f(&[10, 0, 5]));
        assert_eq!(24, f(&[10, 10, 4]));
        assert_eq!(20, f(&[5, 5, 10]));

        // extra rolls after spare/strike
        assert_eq!(20, f(&[10, 6, 4, 1]));
        assert_eq!(24, f(&[10, 10, 4, 1]));
        assert_eq!(15, f(&[5, 5, 5, 1]));
        assert_eq!(15, f(&[1, 9, 5, 1]));
        assert_eq!(15, f(&[10, 0, 5, 1]));
        assert_eq!(20, f(&[5, 5, 10, 1]));

        // extra rolls after open frame
        assert_eq!(9, f(&[4, 5, 5, 1]));
        assert_eq!(5, f(&[5, 0, 5, 1]));
        assert_eq!(0, f(&[0, 0, 5, 1]));

        // invalid inputs
        assert_eq!(1, f(&[1, 10, 0, 1, 1]));
        assert_eq!(2, f(&[1, 10, 1, 1, 1]));
        assert_eq!(11, f(&[5, 10, 5, 1, 1, 1]));
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
            assert_eq!(a.scores().sum::<T>(), b.scores().sum());
        }

        for p in 0..=11 {
            for x in 0..=10 {
                for y in 0..=30 {
                    f(p, std::iter::repeat(x).take(y));
                }
            }
        }
    }

    #[test]
    fn test_normal_frame_to_string() {
        let f = |x: &[T]| mock(x).frame_to_string(0);

        // strike
        assert_eq!(" X ".to_string(), f(&[10, 0]));

        // spare
        assert_eq!("0,/".to_string(), f(&[0, 10]));
        assert_eq!("4,/".to_string(), f(&[4, 6]));
        assert_eq!("5,/".to_string(), f(&[5, 5]));
        assert_eq!("6,/".to_string(), f(&[6, 4]));

        // open frame
        assert_eq!("".to_string(), f(&[]));
        assert_eq!("0,".to_string(), f(&[0]));
        assert_eq!("1,".to_string(), f(&[1]));
        assert_eq!("1,2".to_string(), f(&[1, 2]));
        assert_eq!("0,0".to_string(), f(&[0, 0]));
        assert_eq!("0,4".to_string(), f(&[0, 4]));
        assert_eq!("4,0".to_string(), f(&[4, 0]));
    }

    #[test]
    fn test_last_frame_to_string() {
        let f = |x: &[T]| pad(x).frame_to_string(LAST_FRAME);

        // 1 roll
        assert_eq!("1,".to_string(), f(&[1]));
        assert_eq!("2,".to_string(), f(&[2]));
        assert_eq!("3,".to_string(), f(&[3]));
        assert_eq!("4,".to_string(), f(&[4]));
        assert_eq!("5,".to_string(), f(&[5]));
        assert_eq!("6,".to_string(), f(&[6]));
        assert_eq!("7,".to_string(), f(&[7]));
        assert_eq!("8,".to_string(), f(&[8]));
        assert_eq!("9,".to_string(), f(&[9]));
        assert_eq!("X,".to_string(), f(&[10]));

        // 2 rolls
        assert_eq!("0,0".to_string(), f(&[0, 0]));
        assert_eq!("9,0".to_string(), f(&[9, 0]));
        assert_eq!("9,/,".to_string(), f(&[9, 1]));
        assert_eq!("1,/,".to_string(), f(&[1, 9]));
        assert_eq!("X,0,".to_string(), f(&[10, 0]));
        assert_eq!("X,5,".to_string(), f(&[10, 5]));
        assert_eq!("X,X,".to_string(), f(&[10, 10]));

        // 3 rolls with strikes
        assert_eq!("X,0,0".to_string(), f(&[10, 0, 0]));
        assert_eq!("0,/,0".to_string(), f(&[0, 10, 0]));
        assert_eq!("0,0".to_string(), f(&[0, 0, 0]));
        assert_eq!("0,0".to_string(), f(&[0, 0, 10]));
        assert_eq!("X,X,0".to_string(), f(&[10, 10, 0]));
        assert_eq!("0,/,X".to_string(), f(&[0, 10, 10]));
        assert_eq!("X,X,X".to_string(), f(&[10, 10, 10]));

        // 3 rolls with spares
        assert_eq!("5,/,5".to_string(), f(&[5, 5, 5]));
        assert_eq!("5,/,0".to_string(), f(&[5, 5, 0]));
        assert_eq!("5,0".to_string(), f(&[5, 0, 0]));
        assert_eq!("4,4".to_string(), f(&[4, 4, 4]));
        assert_eq!("4,/,4".to_string(), f(&[4, 6, 4]));
        assert_eq!("6,/,X".to_string(), f(&[6, 4, 10]));
        assert_eq!("4,/,X".to_string(), f(&[4, 6, 10]));
    }
}
