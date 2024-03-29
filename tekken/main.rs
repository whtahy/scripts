use crate::GameState::*;
use read_process_memory::{copy_address, ProcessHandle};
use std::{fmt, io::Write, thread, time::Duration};
use sysinfo::{PidExt, ProcessExt, ProcessRefreshKind, System, SystemExt};
use time::OffsetDateTime;
use windows_sys::Win32::{
    Foundation::CloseHandle,
    System::Diagnostics::ToolHelp::{
        CreateToolhelp32Snapshot, Module32First, MODULEENTRY32,
        TH32CS_SNAPMODULE,
    },
};

type T = usize;

const PROCESS: &str = "TekkenGame-Win64-Shipping.exe";
const DEBUG: [T; 3] = [0x34df554, 0x3524cfe, 0x3524dda];
const LOG: &str = "./log.txt";
const RANKED_START: &str = "enu> Cancel Search";
const RANKED_END: &str = "<#rb> Add to Favor";
const SWAP_HP: &[u8] = &[0, 1, 2];
const SWAP_WINS: &[u8] = &[1, 2, 1];
const TICK_IN_GAME: u64 = 1;
const TICK_OUT_GAME: u64 = 10;

const RANK_MODE: T = 0x34d5dd3;
const STAGE: T = 0x34df550;
const TIMER: [T; 2] = [0x034d6660, 0x48];
const P1_RANK: T = 0x34df54c; // P2_RANK = ?;
const P2_NAME: [T; 4] = [0x034d55a0, 0x0, 0x8, 0x11c];
const CHARACTER: [T; 2] = [0x34f826c, 0x34edf1c]; // [0x34ea8ac, 0x34fb8dc]
const ROUND_WINS: [T; 2] = [0x34cd500, 0x34cd5f0];
const HP: [T; 2] = [0x34ef348, 0x34ebcd8];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum GameState {
    WaitingForProcess,
    GameOpen,
    WaitingForMatch,
    Match,
    GameClosed,
}

#[derive(Debug, PartialEq, Eq)]
struct MatchState {
    matchup: (String, String),
    p1_rank: String,
    wins: (T, T),
    hp: (T, T),
    timer: T,
    stage: String,
    p2_name: String,
}

impl fmt::Display for MatchState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " {} vs {}", self.matchup.0, self.matchup.1)?;
        write!(f, "\n Rank: {}", self.p1_rank)?;
        write!(f, "\n Score: {} - {}", self.wins.0, self.wins.1)?;
        write!(f, "\n P2 name: {}", self.p2_name)?;
        write!(f, "\n Hp: {} - {}", self.hp.0, self.hp.1)?;
        write!(f, "\n Timer: {:?}", self.timer)?;
        write!(f, "\n Stage: {}", self.stage)
    }
}

fn main() {
    daemon();
}

fn daemon() {
    let mut sysinfo = System::new();
    let refresh = ProcessRefreshKind::new();
    let mut start = OffsetDateTime::now_local().unwrap();
    let now = || OffsetDateTime::now_local().unwrap();
    let mut game_state = GameClosed;
    loop {
        sysinfo.refresh_processes_specifics(refresh);
        let pid = match (check_pid(&sysinfo), game_state) {
            (Some(p), GameClosed | WaitingForProcess) => {
                println!("+++ Game open +++");
                game_state = GameOpen;
                p
            }
            (Some(p), _) => p,
            (None, WaitingForProcess) => {
                sleep(TICK_OUT_GAME);
                continue;
            }
            (None, GameClosed) => {
                println!("... Waiting for process ...");
                game_state = WaitingForProcess;
                sleep(TICK_OUT_GAME);
                continue;
            }
            (None, _) => {
                println!("+++ Game closed +++");
                game_state = GameClosed;
                sleep(TICK_OUT_GAME);
                continue;
            }
        };
        let match_state = match (match_state(pid), game_state) {
            (Some(s), GameOpen) => {
                println!("... Waiting for match ...");
                game_state = WaitingForMatch;
                s
            }
            (Some(s), _) => s,
            (None, GameOpen | Match) => {
                println!("... Waiting for match ...");
                game_state = WaitingForMatch;
                sleep(TICK_IN_GAME);
                continue;
            }
            (None, _) => {
                sleep(TICK_IN_GAME);
                continue;
            }
        };
        if game_state == WaitingForMatch && match_start(&match_state) {
            println!("--- Match start ---");
            game_state = Match;
            start = now();
        } else if game_state == Match && match_end(&match_state) {
            println!("--- Match end ---");
            game_state = GameOpen;
            println!("{match_state}");
            println!(
                " {} {} {}",
                date(start),
                time(start),
                duration(start, now())
            );
            log(format!(
                "{},{},{},{},{},{},{},{},{},{}",
                date(start),
                time(start),
                duration(start, now()),
                match_state.p1_rank,
                match_state.matchup.0,
                match_state.matchup.1,
                match_state.wins.0,
                match_state.wins.1,
                match_state.stage,
                match_state.p2_name,
            ));
        } else {
            sleep(TICK_IN_GAME);
        }
    }
}

fn match_state(pid: u32) -> Option<MatchState> {
    let handle = ProcessHandle::try_from(pid).ok()?;
    let base = base_address(pid);
    let rel = |offset: T, n_bytes: T| {
        copy_address(base + offset, n_bytes, &handle).ok()
    };
    let rel_4 = |offset| {
        let bytes = rel(offset, 4)?.try_into().ok()?;
        Some(u32::from_le_bytes(bytes) as T)
    };
    let abs_4 = |addr: T| {
        let bytes = copy_address(addr, 4, &handle).ok()?.try_into().ok()?;
        Some(u32::from_le_bytes(bytes) as T)
    };
    let ptr_chain = |offsets: &[T], n_bytes| {
        let mut iter = offsets.iter().peekable();
        let offset = *iter.next()?;
        let mut ptr = rel_4(offset)?;
        while let Some(offset) = iter.next() {
            ptr += offset;
            match iter.peek() {
                Some(_) => ptr = abs_4(ptr)?,
                None => return copy_address(ptr, n_bytes, &handle).ok(),
            };
        }
        None
    };

    let mode = String::from_utf8(rel(RANK_MODE, RANKED_START.len())?).ok()?;
    if mode != RANKED_START && mode != RANKED_END {
        return None;
    }
    let p2 = ptr_chain(&P2_NAME, 32)?
        .into_iter()
        .filter(|&x| x != 0)
        .collect();
    let [mut p1_char, mut p2_char] = CHARACTER.map(rel_4).map(character);
    let [mut p1_wins, mut p2_wins] = ROUND_WINS.map(rel_4);
    let [mut p1_hp, mut p2_hp] = HP.map(rel_4);

    let rel_1 = |offset: &T| {
        let bytes = rel(*offset, 1)?.try_into().ok()?;
        Some(u8::from_le_bytes(bytes))
    };
    let debug = DEBUG.iter().flat_map(rel_1).collect::<Vec<_>>();
    match &debug[..] {
        SWAP_WINS => {
            std::mem::swap(&mut p1_char, &mut p2_char);
            std::mem::swap(&mut p1_wins, &mut p2_wins);
        }
        SWAP_HP => std::mem::swap(&mut p1_hp, &mut p2_hp),
        _ => {}
    }

    Some(MatchState {
        matchup: (p1_char?, p2_char?),
        p1_rank: rank(rel_4(P1_RANK)?),
        wins: (p1_wins?, p2_wins?),
        hp: (p1_hp?, p2_hp?),
        timer: ptr_chain(&TIMER, 1)?[0] as T,
        stage: stage(rel_4(STAGE)?),
        p2_name: String::from_utf8(p2).ok()?,
    })
}

fn check_pid(sysinfo: &System) -> Option<u32> {
    let pid = sysinfo
        .processes_by_exact_name(PROCESS)
        .next()?
        .pid()
        .as_u32();
    Some(pid)
}

fn log(row: String) {
    let mut file = std::fs::OpenOptions::new().append(true).open(LOG).unwrap();
    writeln!(file, "{}", row).ok();
}

fn date(t: OffsetDateTime) -> String {
    t.date().to_string()
}

fn time(t: OffsetDateTime) -> String {
    format!("{:02}:{:02}", t.hour(), t.minute())
}

fn duration(start: OffsetDateTime, end: OffsetDateTime) -> String {
    let t = (end - start).whole_seconds();
    format!("{}m {}s", t / 60, t % 60)
}

fn match_start(state: &MatchState) -> bool {
    state.timer == 60
        && state.wins == (0, 0)
        && state.hp == (100, 100)
        && state.stage != "Warmup"
}

fn match_end(state: &MatchState) -> bool {
    (state.wins.0 == 3 && state.hp.1 == 0)
        || (state.wins.1 == 3 && state.hp.0 == 0)
}

fn sleep(secs: u64) {
    thread::sleep(Duration::from_secs(secs));
}

fn character(id: Option<T>) -> Option<String> {
    let char = match id? {
        0 => "Paul",
        1 => "Law",
        2 => "King",
        3 => "Yoshimitsu",
        4 => "Hwoarang",
        5 => "Xiaoyu",
        6 => "Jin",
        7 => "Bryan",
        8 => "Heihachi",
        9 => "Kazuya",
        10 => "Steve",
        11 => "Jack-7",
        12 => "Asuka",
        13 => "Devil Jin",
        14 => "Feng",
        15 => "Lili",
        16 => "Dragunov",
        17 => "Leo",
        18 => "Lars",
        19 => "Alisa",
        20 => "Claudio",
        21 => "Katarina",
        22 => "Lucky Chloe",
        23 => "Shaheen",
        24 => "Josie",
        25 => "Gigas",
        26 => "Kazumi",
        28 => "Nina",
        29 => "Master Raven",
        30 => "Lee",
        31 => "Bob",
        32 => "Akuma",
        33 => "Kuma",
        34 => "Panda",
        35 => "Eddy",
        36 => "Eliza",
        37 => "Miguel",
        43 => "Geese",
        44 => "Noctis",
        45 => "Anna",
        46 => "Lei",
        47 => "Marduk",
        48 => "Armor King",
        49 => "Julia",
        50 => "Negan",
        51 => "Zafina",
        52 => "Ganryu",
        53 => "Leroy",
        54 => "Fahkumram",
        55 => "Kunimitsu",
        56 => "Lidia",
        _ => return Some(format!("Unknown character id = {}", id.unwrap())),
    }
    .to_string();
    Some(char)
}

fn rank(id: T) -> String {
    match id {
        0 => "Wood: Beginner",
        1 => "Wood: 9th Kyu",
        2 => "Wood: 8th Kyu",
        3 => "Wood: 7th Kyu",
        4 => "Wood: 6th Kyu",
        5 => "Wood: 5th Kyu",
        6 => "Wood: 4th Kyu",
        7 => "Wood: 3rd Kyu",
        8 => "Wood: 2nd Kyu",
        9 => "Wood: 1st Kyu",
        10 => "Silver: 1st Dan",
        11 => "Silver: 2nd Dan",
        12 => "Silver: 3rd Dan",
        13 => "Blue/Teal: Initiate",
        14 => "Blue/Teal: Mentor",
        15 => "Blue/Teal: Expert",
        16 => "Blue/Teal: Grandmaster",
        17 => "Green: Brawler",
        18 => "Green: Marauder",
        19 => "Green: Fighter",
        20 => "Green: Vanguard",
        21 => "Yellow: Warrior",
        22 => "Yellow: Vindicator",
        23 => "Yellow: Juggernaut",
        24 => "Yellow: Usurper",
        25 => "Orange: Vanquisher",
        26 => "Orange: Destroyer",
        27 => "Orange: Savior",
        28 => "Orange: Overlord",
        29 => "Red: Genbu",
        30 => "Red: Byakko",
        31 => "Red: Seiryu",
        32 => "Red: Suzaku",
        33 => "Purple: Mighty Ruler",
        34 => "Purple: Revered Ruler",
        35 => "Purple: Divine Ruler",
        36 => "Purple: Eternal Ruler",
        37 => "Blue: Fujin",
        38 => "Blue: Raijin",
        39 => "Blue: Yaksa",
        40 => "Blue: Ryujin",
        41 => "Emperor",
        42 => "Tekken King",
        43 => "Tekken God",
        44 => "True Tekken God",
        45 => "Tekken God Prime",
        46 => "Tekken God Omega",
        _ => return format!("Unknown rank id = {}", id),
    }
    .to_string()
}

fn stage(id: T) -> String {
    match id {
        42 => "Warmup",
        // row 1
        0 => "Mishima Dojo",
        1 => "Forgotten Realm",
        2 => "Jungle Outpost",
        53 => "Jungle Outpost 2",
        3 => "Arctic Snowfall",
        4 => "Twilight Conflict",
        54 => "Twilight Conflict 2",
        5 => "Dragon's Nest",
        6 => "Souq",
        7 => "Devil's Pit",
        // row 2
        8 => "Mishima Building",
        9 => "Abandoned Temple",
        30 => "Duomo Di Sirio",
        31 => "Arena",
        32 => "G Corp Helipad (Day)",
        33 => "G Corp Helipad (Night)",
        35 => "Brimstone & Fire",
        36 => "Precipice of Fate",
        37 => "Violet Systems",
        39 => "Kinder Gym",
        41 => "Geometric Plane",
        // row 3
        40 => "Infinite Azure",
        55 => "Infinite Azure 2",
        // DLC
        56 => "Last Day on Earth",
        59 => "Island Paradise",
        51 => "Howard Estate",
        52 => "Hammerhead",
        // ? => "Cave of Enlightenment",
        // ? => "Vermilion Gates",
        _ => return format!("Unknown stage id = {}", id),
    }
    .to_string()
}

fn base_address(pid: u32) -> T {
    let mut module: MODULEENTRY32 = unsafe { std::mem::zeroed() };
    module.dwSize = std::mem::size_of_val(&module) as u32;
    unsafe {
        let handle = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE, pid);
        Module32First(handle, &mut module);
        CloseHandle(handle);
    }
    module.modBaseAddr as T
}
