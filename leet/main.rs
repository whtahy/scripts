fn main() {
    // "0"
    // "1"
    // "2"
    // "000"
    // "111"
    // "222"
    // "000"
    // "002"
    // "001"
    // "010"
    // "020"
    // "0202"
    // "02012"
    let s = "
        00000
        00200
        00000
        02000
        00012
    ";
    println!("{}", crate::coins::coins(s, (3, 3)));
}

pub mod mars {
    use std::collections::{BTreeSet, HashMap, HashSet};

    type T = usize;
    type Coord = (T, T);
    type City = BTreeSet<Coord>;

    pub fn mars(n_kits: T, s: &str) -> T {
        // parse input string into vec
        let v: Vec<&str> = s
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect();

        let n_rows = v.len();
        let n_cols = v[0].len();

        // store resource grid as hashmap: (r, c) -> T
        let mut grid = HashMap::new();
        for (i, line) in v.iter().enumerate() {
            for (j, ch) in line.chars().enumerate() {
                grid.insert((i, j), ch.to_digit(10).unwrap() as T);
            }
        }

        // initialize cities
        let mut cities: HashSet<City> =
            grid.keys().map(|&coord| BTreeSet::from([coord])).collect();

        // greedy city expansion
        let expand = |city: City, new_cities: &mut HashSet<City>| {
            let mut new_coords = Vec::new();
            let mut best = 0;
            for adj in adjacent(&city, n_rows, n_cols) {
                let val = *grid.get(&adj).unwrap();
                if new_coords.is_empty() || val == best {
                    new_coords.push(adj);
                    best = val;
                } else if val > best {
                    new_coords = vec![adj];
                    best = val;
                }
            }
            for coord in new_coords {
                let mut new_city = city.clone();
                new_city.insert(coord);
                new_cities.insert(new_city);
            }
        };

        // for each kit, for each city, expand
        for _ in 1..n_kits {
            let mut new_cities = HashSet::new();
            for city in cities {
                expand(city, &mut new_cities);
            }
            cities = new_cities;
        }

        // total city value
        let city_value = |city: &City| -> T {
            city.iter().flat_map(|coord| grid.get(coord)).sum()
        };

        // find best city
        let best_city = cities
            .iter()
            .max_by(|a, b| city_value(a).cmp(&city_value(b)))
            .unwrap();
        let best_value = city_value(best_city);

        // print input and soln
        let mut pretty_print = Vec::new();
        for (i, row) in v.iter().enumerate() {
            pretty_print.push(String::new());
            pretty_print[i].push_str(row);
            pretty_print[i].push(' ');
            for (j, ch) in row.chars().enumerate() {
                if best_city.contains(&(i, j)) {
                    pretty_print[i].push(ch);
                } else {
                    pretty_print[i].push('*');
                }
            }
        }
        println!("val = {best_value}; N = {} cities", cities.len());
        println!("{}", pretty_print.join("\n"));

        // return value
        best_value
    }

    fn adjacent(city: &City, n_rows: T, n_cols: T) -> HashSet<Coord> {
        let adj = |coord: Coord| {
            let (r, c) = coord;
            [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)]
        };
        city.iter()
            .flat_map(|&coord| adj(coord))
            .filter(|coord| !city.contains(coord))
            .filter(|&(r, c)| r < n_rows && c < n_cols)
            .collect()
    }

    #[macro_export]
    macro_rules! repeat {
        ($x:expr) => {
            for _ in 1..=100 {
                $x
            }
        };
        ($($x:expr;)+) => {
            for _ in 1..=100 {
                $($x;)+
            }
        };
    }

    #[test]
    fn test1() {
        let grid = "
            00090
            00000
            00000
            00000
            22222";
        repeat!(assert_eq!(mars(5, grid), 11));
    }

    #[test]
    fn test2() {
        let grid = "
            20000
            20009
            20000
            20000
            22220";
        repeat!(assert_eq!(mars(5, grid), 11));
    }

    #[test]
    fn test3() {
        let grid = "
            20000
            20000
            20009
            20000
            22202";
        repeat!(assert_eq!(mars(5, grid), 13));
    }

    #[test]
    fn test4() {
        let grid = "
            20000
            20011
            20019
            20000
            22202";
        repeat!(assert_eq!(mars(5, grid), 13));
    }

    #[test]
    fn test5() {
        let grid = "
            03542
            12211
            41363
            10917";
        repeat!(
            assert_eq!(mars(2, grid), 12);
            assert_eq!(mars(3, grid), 18);
            assert_eq!(mars(4, grid), 23);
            assert_eq!(mars(5, grid), 28);
        );
    }

    #[test]
    fn test6() {
        let grid = "
            31031150340119844438948069460996821829480191747552
            01179961811952736134476835008483986961386823831884
            56890269826297507015143365491555377682628700701036
            91192348109381600422126159234245505529951430173921
            88800836536725181221722155406617873302905218609135
            84902349702028081186351805023178562981271666985267
            84813436597759821982467202458875645624028771430079
            56768913033313407072164469699600652179469455106111
            36859338656035455651919025934934187780297724902946
            56202780255718943493247889060198244837917669611498
            75749404567585684692150909275073417501594196630016
            61048198963357468019632772064705971157402575449555
            97621248558928372559597718708634548748967383005676
            30069740273349778385999956615768391942248638730820
            49869196477711695494604651503867448002737330283121
            33136938982419715065013591015453681450036606503142
            19034425008206186663953321322036314582120112724299
            16243602983838714739938767473020210051784223884272
            46945299001446646183134109746770321928424763007183
            87886541190636031109587356005035371049185081130922
            33383862729056113784577783385936363148331688597698
            82286490268580434919272134090971272024137650936882
            45985656348334729647742832994437912832274869963097
            88059553798675635264619289674812068430115117494347
            60037619133910226155634017397993602532190499262494
            89877616642093855140864184278794792656242270453079
            87745400096364916558717493627702406745238005353512
            57479290956829189962203626635385040376696054298858
            56671132197718474226443618183513577760289582390746
            88516474896699863713659774175117355344099787517864
            06393374411494318381691073504692836019201254088485
            01056154054945307356080711011225626660025547279556
            74625405612902315254983106734338330807594817389849
            62528024227706566633616581810517058839903015244586
            59084999376660412140880926161598567409200783397316
            52272285574079003527399319150954413369160396215646
            38755472548972613028869706139600232130283711048916
            80319476270362429421740844486171027844423912514459
            59548702873494750928184530210466136422790688954996
            12963768249164333105442554064436992616024513709851
            66191836198666541174053086647379823180375974657292
            76029552290586068954464803309910705004106478571611
            40115254985439966901197252646082743223255076670829
            41372456854444596994281626636306043933701127779729
            73605283538722294832658129119683191379438219985664
            42175578605850534259255062455451659013541822341154
            38505072390865444718760476810941615797072822334866
            48836990139011033782860147684125221823999898877571
            25229602187436739365518254778370546904950695883814
            14697034517070965040914690846840673633241783936678";
        assert_eq!(mars(8, grid), 70);
        assert_eq!(mars(9, grid), 78);
        assert_eq!(mars(10, grid), 86);
    }
}

pub mod coins {
    use std::collections::{HashMap, HashSet};

    type Coord = (usize, usize);
    type T = isize;
    type Maze = HashMap<Coord, char>;

    const START: Coord = (0, 0);
    const WALL: char = '1';
    const COIN: char = '2';

    pub fn coins(s: &str, end: Coord) -> T {
        if end == START {
            panic!()
        }

        // parse input string into vec
        let v: Vec<&str> = s
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect();

        // store maze as hashmap: (r, c) -> char
        // store coin coords
        let mut maze = HashMap::new();
        let mut coins = Vec::new();
        for (i, line) in v.iter().enumerate() {
            for (j, ch) in line.chars().enumerate() {
                let coord = (i, j);
                if ch != WALL {
                    maze.insert(coord, ch);
                }
                if ch == COIN && coord != end {
                    coins.push(coord);
                }
            }
        }

        let mut no_path = false;

        // coin graph helper function
        let mut coin_graph = HashMap::new();
        let mut insert = |start: Coord, end: Coord| {
            let distance = bfs(&maze, start, end);
            if distance == -1 {
                no_path = true;
            } else {
                coin_graph.insert((start, end), distance);
            }
        };

        // create coin graph: (coin a, coin b) -> distance
        insert(START, end);
        for &coord_a in coins.iter() {
            // start -> coin
            insert(START, coord_a);
            // coin -> end
            insert(end, coord_a);
            // coin -> coin
            for &coord_b in coins.iter().filter(|&&c| c != coord_a) {
                insert(coord_a, coord_b);
            }
        }

        // early return: can't reach coin or exit
        if no_path {
            return -1;
        }

        let dist = |start: Coord, end: Coord| {
            coin_graph
                .get(&(start, end))
                .or_else(|| coin_graph.get(&(end, start)))
                .unwrap()
        };
        let path_len = |coins: &[Coord]| -> T {
            dist(START, coins[0])
                + dist(coins[coins.len() - 1], end)
                + coins
                    .windows(2)
                    .map(|w| match w {
                        &[a, b] => dist(a, b),
                        _ => panic!(),
                    })
                    .sum::<T>()
        };

        // early return: 0 or 1 coin
        if coins.is_empty() {
            return *dist(START, end);
        } else if coins.len() == 1 {
            return *dist(START, coins[0]) + *dist(coins[0], end);
        }

        let mut best = T::MAX;
        let mut c = vec![0; coins.len()];
        let mut i = 1;
        while i < coins.len() {
            let p = path_len(&coins);
            if p < best {
                best = p;
            };
            permute(&mut coins, &mut c, &mut i);
        }

        best
    }

    fn bfs(maze: &Maze, start: Coord, end: Coord) -> T {
        if start == end {
            return 0;
        }

        let mut paths = HashSet::from([start]);
        let mut history = HashSet::new();
        let mut path_len = 0;

        let adj = |coord: Coord| {
            let (r, c) = coord;
            [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)]
        };

        loop {
            let mut new_paths = HashSet::new();
            let mut adjacent = paths
                .iter()
                .flat_map(|&coord| adj(coord))
                .filter(|coord| !paths.contains(coord))
                .filter(|coord| !history.contains(coord))
                .filter(|coord| maze.get(coord).is_some())
                .peekable();
            if adjacent.peek().is_none() {
                return -1;
            }
            path_len += 1;
            for coord in adjacent {
                if coord == end {
                    return path_len;
                }
                new_paths.insert(coord);
            }
            history.extend(paths);
            paths = new_paths;
        }
    }

    // Heap's algorithm: https://en.wikipedia.org/wiki/Heap%27s_algorithm
    fn permute(v: &mut [Coord], c: &mut [usize], i: &mut usize) {
        while *i < v.len() && c[*i] >= *i {
            c[*i] = 0;
            *i += 1;
        }
        if *i >= v.len() {
            return;
        } else if *i % 2 == 0 {
            v.swap(*i, 0);
        } else {
            v.swap(*i, c[*i]);
        }
        c[*i] += 1;
        *i = 1;
    }
}

// directions: N, S, E, W only
// find shortest route
// return LENGTH of shortest path, or -1 if NOT possible

// n x m array
// n, m <= 100
// # coins: 0 <= n <= 10
// values are 0, 1, or 2
// coord = (row, col)

// eg
// 020
// 122
// 130
// 100
