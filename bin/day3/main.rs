use itertools::Itertools;

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.trim()
                .split_whitespace()
                .filter_map(|el| el.parse::<u32>().ok())
                .sorted()
                .collect::<Vec<u32>>()
        })
        .filter(|x| x[0] + x[1] > x[2])
        .count()
}
fn part2(input: &str) -> usize {
    let all_int: Vec<u32> = input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse::<u32>().ok())
        .collect();

    all_int
        .chunks(9)
        .map(|chunk| {
            chunk
                .iter()
                .enumerate()
                .fold(vec![vec![], vec![], vec![]], |mut acc, (i, &val)| {
                    acc[i % 3].push(val);
                    acc
                })
        })
        .flatten()
        .map(|mut vec| {
            vec.sort();
            vec
        })
        .filter(|vec| vec[0] + vec[1] > vec[2])
        .count()
}
fn main() {
    let input = include_str!("input3.txt");
    println!("{}, {}", part1(input), part2(input));
}