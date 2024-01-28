use std::collections::HashMap;
use itertools::Itertools;

fn decrypt(name: &str, id: u32) -> String {
    name.chars()
        .map(|c| match c {
            'a'..='z' => {
                let shift = (id % 26) as u8;
                let new_c = ((c as u8 - 'a' as u8 + shift) % 26) + 'a' as u8;
                new_c as char
            }
            '-' => ' ',
            _ => c,
        })
        .collect()
}

fn sort_keys_by_value(map: HashMap<char, i32>) -> Vec<char> {
    map
    .into_iter()
    .map(|(k, v)| (std::cmp::Reverse(v), k))
    .sorted_unstable()
    .take(5)
    .map(|(_k, v)| v)
    .collect::<Vec<char>>()
}

fn part1(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut map = HashMap::new();
        let (enc_name, rest) = line.rsplit_once('-').unwrap();
        let (sec_id_str, checksum_str) = rest.split_once('[').unwrap();
        let checksum: Vec<_> = checksum_str.trim_end_matches(']').chars().collect();
        let sec_id = sec_id_str.parse::<u32>().unwrap();
        
        for c in enc_name.replace("-", "").chars() {
            *map.entry(c).or_insert(0) += 1;
        }
        
        let sorted_values = sort_keys_by_value(map);
        let is_real = sorted_values.iter().zip(checksum.iter())
            .all(|(&st, &ch)| st == ch);
        
        if is_real {
            sum += sec_id;
        }
    }
    sum
}

fn part2(input: &str) -> u32 {
    input.lines()
        .filter_map(|line| {
            let (left, _) = line.split_once('[').unwrap();
            let (enc_name, id_str) = left.rsplit_once('-').unwrap();
            let id: u32 = id_str.parse().unwrap();
            let decrypted_name = decrypt(enc_name, id);
            if decrypted_name.contains("object") {
                Some(id)
            } else {
                None
            }
        })
        .next()
        .unwrap_or(0)
}


fn main() {
    let input = include_str!("input4.txt");
    println!("{}, {}", part1(input), part2(input));
}