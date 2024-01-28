#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused)]
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet, BinaryHeap},
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
        self.path.len().cmp(&other.path.len())
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cmp(&self))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        other.crds == self.crds
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
        let mut grid: Vec<Vec<char>> = input.lines()
            .enumerate()
            .map(|(y, l)| l.chars().enumerate().map(|(x, c)| {
                if c == 'S' { st = (y, x) } else if c == 'V' { end = (y - 1, x - 1)}
                c
            }).collect()).collect();
        Self { plan: grid, st, end }
    }
    fn find_path(&self, salt: &str) -> String {
        let state = State::new(self.st, String::new());
        let mut q = BinaryHeap::new();
        q.push(state);
        while let Some(State{ crds: (y, x) , path }) = q.pop() {
            if (y, x) == self.end { return path }
            let open = hash([salt, path.as_str()].concat());
            let valid = self.nghrs(y, x, open);
            for (yn, xn, c) in valid {
                let mut new_path = path.clone();
                new_path.push(c);
                q.push(State::new((yn, xn), new_path));
            }
        }
        !unreachable!()
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

fn part1(passcode: &str) -> String {
    let input = include_str!("input_lib.txt");
    let mut grid = Grid::new(input);
    grid.find_path(passcode)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!("DDRRRD".to_string(), part1("ihgpwlah"));
    }
    #[test]
    fn test_2() {
        assert_eq!("DDUDRLRRUDRD".to_string(), part1("kglvqrro"));
    }
    #[test]
    fn test_3() {
        assert_eq!("DRURDRUDDLLDLUURRDULRLDUUDDDRR".to_string(), part1("ulqzkmiv"));
    }
}