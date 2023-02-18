use std::path::Path;
use std::process::Command;
use std::{env, fs};

const INVALID_COOKIE: &str =
    "Puzzle inputs differ by user.  Please log in to get your puzzle input.";

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if let [_, year, day] = &args[..] {
        let path = format!("z:/git/advent_of_code/aoc_{year}/day{day}.txt");
        if Path::new(&path).exists() {
            return println!("File already exists: {path}");
        }
        let cookie =
            env::var("aoc_session_cookie")
            .expect("Missing aoc_session_cookie env var.");
        let url = format!("https://adventofcode.com/{year}/day/{day}/input");
        let curl = format!("curl -b session={cookie} {url} > {path}");
        Command::new("bash").arg("-c").arg(curl).output().unwrap();
        if Path::new(&path).exists() {
            let puzzle = fs::read_to_string(&path).unwrap();
            if puzzle.trim() == INVALID_COOKIE {
                fs::remove_file(path).ok();
                return println!("Invalid cookie!");
            }
            let crlf = puzzle.replace("\r\n", "\n").replace('\n', "\r\n");
            fs::write(&path, crlf).unwrap();
            return println!("Download success: {path}");
        } else {
            return println!("Download fail: {path}");
        }
    }
    println!("Usage: aoc <year> <day>")
}
