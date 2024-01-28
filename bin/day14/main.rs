use itertools::Itertools;

#[derive(Debug, Clone)]
struct Pad<'a> {
    keys: Vec<(usize, char)>,
    count: u32,
    salt: &'a str,
}

impl<'a> Pad<'a> {
    fn new(salt: &'a str) -> Self {
        Pad {
            keys: Vec::new(),
            count: 0,
            salt,
        }
    }
    fn hash(&mut self, laps: usize) -> usize {
        for idx in 0.. {
            let hash = self.compute_hash(idx, laps);

            let chars: Vec<char> = hash.chars().collect();

            let mut invalid: Vec<usize> = Vec::new();

            for &(key_idx, key_char) in &self.keys {
                if idx - key_idx > 1000 {
                    invalid.push(key_idx);
                } else if chars.windows(5).any(|w| w.iter().all_equal() && w[0] == key_char) {
                    self.count += 1;
                    if self.count == 64 { return key_idx }
                    invalid.push(key_idx);
                }
            }

            self.keys.retain(|&(k, _)| !invalid.contains(&k));

            if let Some(c) = chars.windows(3)
            .find(|w| w.iter().all_equal())
            .map(|w| w[0]) {
                self.keys.push((idx, c));
            }
        }
        0
    } 
    fn compute_hash(&self, idx: usize, laps: usize) -> String {
        let mut hash = md5::compute(format!("{}{}", self.salt, idx)).to_vec();
        let mut hxd = to_hxd(hash);
    
        for _ in 0..laps {
            hash = md5::compute(hxd).to_vec();
            hxd = to_hxd(hash);
        }
    
        hxd
    }
}

fn to_hxd(v: Vec<u8>) -> String {
    v.iter().map(|byte| format!("{:02x}", byte)).collect::<String>()
}

fn part1(input: &str) -> usize {
    let mut pad = Pad::new(input);
    pad.hash(0)
}

fn part2(input: &str) -> usize {
    let mut pad = Pad::new(input);
    pad.hash(2016)
}

fn main() {
    println!("{}", part2("zpqevtbw"));
}