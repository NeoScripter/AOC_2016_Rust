const TARGET_CHIPS: [u32; 2] = [17, 61];
use std::collections::HashMap;

fn part1(input: &str) -> u32 {
    let mut bots = HashMap::new();
    let mut instructions = Vec::new();
    let mut outputs: HashMap<u32, Vec<u32>> = HashMap::new();
    input.lines().for_each(|line| {
        if line.starts_with("value") {
            let mut numbers = line.split_whitespace().filter_map(|x| x.parse::<u32>().ok());
            let value = numbers.next().unwrap();
            let bot = numbers.next().unwrap();
            bots.entry(bot).or_insert_with(Vec::new).push(value);
        } else {
            instructions.push(line);
        }
    });
    let mut target_bot = 0;
    while target_bot == 0 {
        for inst in instructions.iter() {
            let parts: Vec<&str> = inst.split_whitespace().collect();
            let giver = parts[1].parse::<u32>().unwrap();
            let recipient_1 = parts[6].parse::<u32>().unwrap();
            let recipient_2 = parts[11].parse::<u32>().unwrap();
            let mut chips = Vec::new();
            if let Some(two_chips) = bots.get_mut(&giver) {
                if two_chips.len() == 2 {
                    chips.append(two_chips);
                    chips.sort_unstable();
                    let (low, high) = (chips[0], chips[1]);
                    if chips == TARGET_CHIPS {
                        target_bot = giver;
                        break;
                    }
                    match parts[5] {
                        "bot" => bots.entry(recipient_1).or_insert_with(Vec::new).push(low),
                        _ => outputs.entry(recipient_1).or_insert_with(Vec::new).push(low),
                    }
                    match parts[10] {
                        "bot" => bots.entry(recipient_2).or_insert_with(Vec::new).push(high),
                        _ => outputs.entry(recipient_2).or_insert_with(Vec::new).push(high),
                    }
                }
            }
        }
    }
    target_bot
}

fn part2(input: &str) -> u32 {
    let mut bots = HashMap::new();
    let mut instructions = Vec::new();
    let mut outputs: HashMap<u32, Vec<u32>> = HashMap::new();
    input.lines().for_each(|line| {
        if line.starts_with("value") {
            let mut numbers = line.split_whitespace().filter_map(|x| x.parse::<u32>().ok());
            let value = numbers.next().unwrap();
            let bot = numbers.next().unwrap();
            bots.entry(bot).or_insert_with(Vec::new).push(value);
        } else {
            instructions.push(line);
        }
    });
    while !(0..=2).all(|key| outputs.contains_key(&key)) {
        for inst in instructions.iter() {
            let parts: Vec<&str> = inst.split_whitespace().collect();
            let giver = parts[1].parse::<u32>().unwrap();
            let recipient_1 = parts[6].parse::<u32>().unwrap();
            let recipient_2 = parts[11].parse::<u32>().unwrap();
            let mut chips = Vec::new();
            if let Some(two_chips) = bots.get_mut(&giver) {
                if two_chips.len() == 2 {
                    chips.append(two_chips);
                    chips.sort_unstable();
                    let (low, high) = (chips[0], chips[1]);
                    match parts[5] {
                        "bot" => bots.entry(recipient_1).or_insert_with(Vec::new).push(low),
                        _ => outputs.entry(recipient_1).or_insert_with(Vec::new).push(low),
                    }
                    match parts[10] {
                        "bot" => bots.entry(recipient_2).or_insert_with(Vec::new).push(high),
                        _ => outputs.entry(recipient_2).or_insert_with(Vec::new).push(high),
                    }
                }
            }
        }
    }
    outputs.into_iter().filter(|&(key, _)| (0..=2).contains(&key)).flat_map(|(_, v)| v).product()

}

fn main() {
    let input = include_str!("input10.txt");
    println!("{}", part2(input));
}