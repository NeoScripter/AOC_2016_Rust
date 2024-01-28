
fn part1(input: &str) -> usize {
    let mut total_length = 0;
    let mut chars = input.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '(' {
            let mut marker = String::new();
            while let Some(&next_char) = chars.peek() {
                chars.next();
                if next_char == ')' {
                    break;
                }
                marker.push(next_char);
            }
            let parts: Vec<&str> = marker.split('x').collect();
            let n: usize = parts[0].parse().unwrap();
            let m: usize = parts[1].parse().unwrap();

            let skipped_chars: String = chars.by_ref().take(n).collect();
            total_length += skipped_chars.len() * m;
        } else {
            total_length += 1;
        }
    }

    total_length
}
fn part2(input: &str) -> u128 {
    let mut total_length = 0;
    let mut chars = input.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '(' {
            let mut marker = String::new();
            while let Some(&next_char) = chars.peek() {
                chars.next();
                if next_char == ')' {
                    break;
                }
                marker.push(next_char);
            }
            let parts: Vec<&str> = marker.split('x').collect();
            let n: u128 = parts[0].parse().unwrap();
            let m: u128 = parts[1].parse().unwrap();

            let skipped_chars: String = chars.by_ref().take(n as usize).collect();
            let decompressed = part2(&skipped_chars) * m;
            total_length += decompressed;
        } else {
            total_length += 1;
        }
    }

    total_length
}

fn main() {
    let input = include_str!("input9.txt");
    println!("{}", part2(input));
}