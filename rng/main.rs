const P0: u64 = 0xa0761d6478bd642f;
const P1: u128 = 0xe7037ed1a0b428db;

struct Rng {
    seed: u64,
}

impl Iterator for Rng {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.seed += P0;
        let b = self.seed as u128;
        let a = b ^ P1;
        let c = a * b;
        let next = (c >> 64) ^ c;
        Some(next as u64)
    }
}

fn main() {
    let mut rng = Rng { seed: 123 };

    let n_lines = 100000;
    let rolls = (5, 40);
    let max_pins = 11;

    let pins = |x| x % (max_pins + 1);
    let n_rolls = |x| x % (rolls.1 - rolls.0 + 1) + rolls.0;

    for _ in 1..=n_lines {
        let n = n_rolls(rng.next().unwrap()) as usize;
        let s: String = rng
            .by_ref()
            .take(n)
            .map(|x| pins(x).to_string() + " ")
            .collect();
        println!("{}", s.trim_end());
    }
}
