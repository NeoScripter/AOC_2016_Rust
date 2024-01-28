use std::{
    collections::BinaryHeap,
    cmp::Ordering,
    fmt,
};

#[derive(Debug, Clone, Eq)]
struct State {
    crds: (usize, usize),
    path: String,
}

impl State {
    fn new(crds: (usize, usize), path: String) -> Self {
        Self { crds, path }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.path.len().cmp(&self.path.len())
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.crds == other.crds
    }
}

#[derive(Debug, Clone)]
struct Grid {
    plan: Vec<Vec<char>>,
    st: (usize, usize),
    end: (usize, usize),
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.plan {
            for c in row {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    fn new(input: &str) -> Self {
        let (mut st, mut end) = ((0, 0), (0, 0));
        let grid: Vec<Vec<char>> = input.lines()
            .enumerate()
            .map(|(y, l)| l.chars().enumerate().map(|(x, c)| {
                if c == 'S' { st = (y, x) } else if c == 'V' { end = (y - 1, x - 1)}
                c
            }).collect()).collect();
        Self { plan: grid, st, end }
    }
    fn find_path(&self, salt: &str) -> (String, usize) {
        let state = State::new(self.st, String::new());
        let mut q = BinaryHeap::new();
        let (mut min, mut max) = (String::new(), 0);
        q.push(state);
        while let Some(State{ crds: (y, x) , path }) = q.pop() {
            if (y, x) == self.end { 
                if min.is_empty() { min = path.clone() }
                max = max.max(path.len()); 
                continue 
            }
            let open = hash([salt, path.as_str()].concat());
            let valid = self.nghrs(y, x, open);
            for (yn, xn, c) in valid {
                let mut new_path = path.clone();
                new_path.push(c);
                q.push(State::new((yn, xn), new_path));
            }
        }
        (min, max)
    }
    fn nghrs(&self, y: usize, x: usize, open: Vec<char>) -> Vec<(usize, usize, char)> {
        let mut valid = Vec::new();
        for c in open {
            match c {
                'U' => if self.plan[y-1][x] != '#' {valid.push((y - 2, x, c))},
                'D' => if self.plan[y+1][x] != '#' {valid.push((y + 2, x, c))},
                'L' => if self.plan[y][x-1] != '#' {valid.push((y, x - 2, c))},
                _ => if self.plan[y][x+1] != '#' {valid.push((y, x + 2, c))},
            }
        }
        valid
    }
}

const OPEN: &[char] = &['b', 'c', 'd', 'e', 'f'];
const DIRS: &[char] = &['U', 'D', 'L', 'R'];

fn hash(s: String) -> Vec<char> {
    let digest = md5::compute(s.as_bytes());
    let hex_string = format!("{:x}", digest);

    hex_string.chars()
        .take(4)
        .zip(DIRS.iter())
        .filter_map(|(c, &d)| if OPEN.contains(&c) { Some(d) } else { None })
        .collect()
}

fn solve(passcode: &str) -> (String, usize) {
    let input = include_str!("input17.txt");
    let grid = Grid::new(input);
    let (p1, p2) = grid.find_path(passcode);
    (p1, p2)
}
fn main() {
    let (p1, p2) = solve("vkjiggvb");
    println!("{}, {}", p1, p2);
}