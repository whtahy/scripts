use std::env;
use std::path::Path;
use std::process::Command;

fn arg(i: usize) -> String {
    env::args().nth(i).unwrap()
}

// use env variable ADVENT_OF_CODE_FLAGS
// to get download path (instead of hardcode)
// create env variable helper?

// add --crlf option

fn main() {
    let year = arg(1);
    let day = arg(2);
    let cookie = env::var("aoc_session_cookie").unwrap();

    let path = format!("c:/git/advent_of_code/input/{0}/day{1}.txt", year, day);

    if !Path::new(&path).exists() {
        let url = format!("https://adventofcode.com/{0}/day/{1}/input", year, day);
        let curl = format!("curl -b session={0} {1} > {2}", cookie, url, path);
        Command::new("bash").arg("-c").arg(curl).output().unwrap();
    } else {
        println!("File already exists.")
    }
}
