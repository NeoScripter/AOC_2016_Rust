use std::str::FromStr;
use md5;

fn part1(input: &str) -> String {
    let mut answer = String::new();

    for n in 1.. {
        let concatenated_string = format!("{}{}", input, n);
        let md5_hash = md5::compute(concatenated_string);
        let hex_representation: String = md5_hash.iter().map(|byte| format!("{:02x}", byte)).collect();

        if hex_representation.starts_with("00000") {
            if let Some(digit) = hex_representation.chars().nth(5) {
                answer.push(digit);
            }
        }

        if answer.len() == 8 {
            break;
        }
    }
    answer
}

fn part2(input: &str) -> String {
    let mut answer: Vec<Option<String>> = vec![None; 8];

    for n in 1.. {
        let concatenated_string = format!("{}{}", input, n);
        let md5_hash = md5::compute(concatenated_string);
        let hex_representation: String = md5_hash.iter().map(|byte| format!("{:02x}", byte)).collect();

        if hex_representation.starts_with("00000") {
            if let Ok(pos) = usize::from_str(&hex_representation[5..6]) {
                if pos <= 7 && answer[pos].is_none() {
                    answer[pos] = Some(hex_representation[6..7].to_string());
                }
            }
        }

        if answer.iter().all(Option::is_some) {
            break;
        }
    }

    answer.into_iter().filter_map(|x| x).collect()
}

fn main() {
    let input = "ugkcyxxp";
    println!("{}, {}", part1(input), part2(input));
}