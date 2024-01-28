use itertools::Itertools;
use std::collections::HashMap;

fn run(input: &str) -> (String, String) {
    let init_input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let width = init_input.get(0).map_or(0, Vec::len);

    let (part1, part2): (Vec<char>, Vec<char>) = (0..width).map(|i| {
        let mut map = HashMap::new();
        for row in &init_input {
            *map.entry(row[i]).or_insert(0) += 1;
        }
        map.into_iter()
            .minmax_by(|a, b| a.1.cmp(&b.1))
            .into_option()
            .map(|((min_key, _), (max_key, _))| (max_key, min_key))
            .unwrap_or(('-', '-'))
    }).unzip();

    (part1.into_iter().collect(), part2.into_iter().collect())
}

fn main() {
    let input = include_str!("input6.txt");
    let (part1, part2) = run(input); 
    println!("{}, {}", part1, part2);
}