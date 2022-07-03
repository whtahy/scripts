use std::collections::{HashMap, HashSet};

type T = usize;
type Coord = (T, T);
type City = Vec<Coord>;

struct Grid {
    n_rows: T,
    n_cols: T,
    tiles: HashMap<Coord, T>,
}

impl Grid {
    fn get(&self, coord: &Coord) -> T {
        *self.tiles.get(coord).unwrap()
    }
}

fn main() {
    let fmt = |s: &str| -> String {
        s.lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.trim())
            .collect::<Vec<_>>()
            .join("\n")
    };
    let grid = fmt("
        00900
        00000
        00000
        00000
        22222
        ");
    mars(5, 5, 5, &grid);
    let grid = fmt("
        20000
        20000
        20009
        20000
        20000
        ");
    mars(5, 5, 5, &grid);
}

fn mars(n_rows: T, n_cols: T, n_kits: T, s: &str) {
    let mut tiles = HashMap::new();
    for (i, line) in s.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            tiles.insert((i, j), ch.to_digit(10).unwrap() as T);
        }
    }

    let grid = Grid {
        n_rows,
        n_cols,
        tiles,
    };

    let mut cities: Vec<City> =
        grid.tiles.keys().map(|&coord| vec![coord]).collect();

    for _ in 1..n_kits {
        cities = cities
            .iter()
            .flat_map(|city| expand(city, &grid))
            .collect::<Vec<_>>();
    }

    let value =
        |city: &City| city.iter().map(|coord| grid.get(coord)).sum::<T>();
    let best = cities
        .iter()
        .max_by(|x, y| value(x).cmp(&value(y)))
        .unwrap();

    println!("\nGrid\n---");
    print_grid(&grid);
    println!("\nSolution\n---");
    print_city(best, &grid);
}

fn expand(city: &City, grid: &Grid) -> HashSet<City> {
    let mut new = HashSet::new();
    let mut best = 0;
    for coord in city.iter() {
        for adj in adjacent(*coord, grid) {
            if city.contains(&adj) {
                continue;
            } else if new.is_empty() || grid.get(&adj) == best {
                new.insert(adj);
            } else if grid.get(&adj) > best {
                new = HashSet::from([adj]);
                best = grid.get(&adj);
            }
        }
    }

    new.into_iter()
        .map(|coord| {
            let mut city = city.clone();
            city.push(coord);
            city
        })
        .collect()
}

fn adjacent(coord: Coord, grid: &Grid) -> impl Iterator<Item = Coord> + '_ {
    let (r, c) = coord;
    vec![(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)]
        .into_iter()
        .filter(|&(r, c)| r < grid.n_cols && c < grid.n_rows)
}

fn print_grid(grid: &Grid) {
    for i in 0..grid.n_rows {
        for j in 0..grid.n_cols {
            print!("{}", grid.tiles.get(&(i, j)).unwrap());
        }
        println!();
    }
}

fn print_city(city: &City, grid: &Grid) {
    for i in 0..grid.n_rows {
        for j in 0..grid.n_cols {
            if city.contains(&(i, j)) {
                print!("*");
            } else {
                print!("{}", grid.tiles.get(&(i, j)).unwrap());
            }
        }
        println!();
    }
}
