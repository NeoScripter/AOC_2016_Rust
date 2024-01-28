use std::collections::HashMap;

fn part1(input: &str) -> i32 {
    let mut rgs: HashMap<&str, i32> = [("a", 0), ("b", 0), ("c", 0), ("d", 0)].into_iter().collect();
    let ins: Vec<Vec<&str>> = input.lines().map(|line| line.split_whitespace().collect::<Vec<&str>>()).collect();
    let mut idx = 0;
    while idx < ins.len() {
        match ins[idx][0] {
            "cpy" => {
                let value = match rgs.get(ins[idx][1]) {
                    Some(&v) => v,
                    None => ins[idx][1].parse::<i32>().unwrap(),
                };
                *rgs.get_mut(ins[idx][2]).unwrap() = value;
            },
            "inc" => *rgs.get_mut(ins[idx][1]).unwrap() += 1,
            "dec" => *rgs.get_mut(ins[idx][1]).unwrap() -= 1,
            _ => {
                let ind = match rgs.get(ins[idx][1]) {
                    Some(&x) => x,
                    None => ins[idx][1].parse::<i32>().unwrap(),
                };
                if ind != 0 {
                    let offset = ins[idx][2].parse::<isize>().unwrap();
                    let new_idx = idx as isize;
                    if new_idx + offset < 0 {
                        break
                    }
                    idx = (new_idx + offset) as usize;
                    continue;
                }
            }
        }
        idx += 1;
    }
    *rgs.get("a").unwrap()
}
fn main() {
    let input = include_str!("input12.txt");
    println!("{}", part1(input));
}