// Part 2 takes about 2 minutes to run

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct State (HashMap<u64, char>);

impl State {
    fn new(num: Vec<char>) -> Self {
        let map = num.into_iter().enumerate().map(|(i, c)| ((i + 1) as u64, c)).collect();
        Self(map)
    }
    fn expand(&mut self) {
        let len = self.0.len() as u64;
        self.0.insert(len + 1, '0');

        for n in (1..=len).rev() {
            let next = if self.0[&n] == '1' { '0' } else { '1' };
            self.0.insert(2 * len + 2 - n, next);
        }
    }
    fn append(&mut self, len: usize) {
        (len + 1..=self.0.len()).for_each(|i| { self.0.remove(&(i as u64)); })
    }
    fn checksum(&mut self) {
        let mut new = HashMap::new();
        let mut last = 'c';
        for n in 1..=self.0.len() as u64 {
            if n % 2 == 0 {
                if self.0[&n] == last { new.insert(n/2, '1'); }
                else { new.insert(n/2, '0'); }
            } else { last = self.0[&n] }
        }
        self.0 = new;
    }
    fn print_map(&self) {
        let mut s = String::new();
        for n in 1..=self.0.len() as u64 { s.push(self.0[&n])}
        println!("{}", s);
    }
}

fn solve(len: usize) {
    let input = "10111100110001111";
    let num: Vec<char> = input.chars().collect();
    let mut state = State::new(num);
    while state.0.len() < len { state.expand() }
    state.append(len);
    state.checksum();
    while state.0.len() % 2 == 0 { state.checksum() }
    state.print_map();
}

fn main() {
    solve(272);
}