use std::collections::HashSet;
use itertools::Itertools;

fn is_abba(slice: &str) -> bool {
    slice.chars().tuple_windows()
    .any(|(one, two, three, four)| one == four && two == three && one != two)
}

fn part1(input: &str) -> usize {
    input.lines().filter(|&line| {
        let parts: Vec<&str> = line.split(|c| c == '[' || c == ']').collect();
        let (mut outside_brackets, mut inside_brackets) = (false, true);
        for (idx, &part) in parts.iter().enumerate() {
            if idx % 2 == 0 {
                outside_brackets |= is_abba(part);
            } else {
                inside_brackets &= !is_abba(part);
            }
        }
        outside_brackets && inside_brackets
    }).count()
}
fn part2(input: &str) -> usize {
    input.lines().filter(|&line| {
        let parts: Vec<&str> = line.split(|c| c == '[' || c == ']').collect();
        let (supernet, hypernet): (Vec<_>, Vec<_>) = parts.into_iter().enumerate().partition(|(i, s)| i % 2 == 0);
        let mut seen = HashSet::new();
        let mut bool = false;
        for (_i, aba) in &supernet {
            aba.chars().collect::<Vec<char>>().windows(3).for_each(|win| {
                if win[0] == win[2] && win[0] != win[1] {
                    seen.insert((win[1], win[0], win[1]));
                }
            });
        }
    
        for (_i, bab) in &hypernet {
            bab.chars().collect::<Vec<char>>().windows(3).for_each(|win| {
                if win[0] == win[2] && win[0] != win[1] && seen.contains(&(win[0], win[1], win[2])) {
                    bool = true;
                }
            });
        }
        bool
    }).count()
}
fn main() {
    let input = include_str!("input7.txt");
    println!("{}", part1(input));
}