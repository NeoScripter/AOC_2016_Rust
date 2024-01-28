#![allow(dead_code)]
#![allow(unused_variables)]
use nom::{
    bytes::complete::take_till,
    IResult,
};
use std::fmt;

const WIDTH: usize = 50;
const HEIGHT: usize = 6;
#[derive(Debug)]
struct Screen(Vec<Vec<char>>);

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.0 {
            for &num in row {
                write!(f, "{: >1}", num)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
impl Screen {
    fn new() -> Self {
        Screen(vec![vec!['.'; WIDTH]; HEIGHT])
    }
    fn rect(&mut self, width: usize, height: usize) {
        for y in 0..height {
            for x in 0..width {
                self.0[y][x] = '#'
            }
        }
    }
    fn rotate_row(&mut self, row_num: usize, dist: usize) {
        self.0[row_num].rotate_right(dist)
    }
    fn rotate_col(&mut self, col_num: usize, dist: usize) {
        let temp_col: Vec<_> = (0..6).map(|i| self.0[(i + 6 - dist) % 6][col_num]).collect();
    
        for (i, &val) in temp_col.iter().enumerate() {
            self.0[i][col_num] = val;
        }
    }
    fn count_lit(&self) -> usize {
        self.0.iter().map(|rows| rows.iter().filter(|&&c| c == '#').count()).sum::<usize>()
    }
}

fn parse_line(line: &str) -> IResult<&str, &str> {
    take_till(|c: char| c.is_digit(10))(line)
}

fn part1(input: &str) -> usize {
    let mut screen = Screen::new();
    for line in input.lines() {
        if let Ok((rest, start)) = parse_line(line) {
            match start.trim() {
                "rect" => {
                    let (width, height) = rest.split_once("x").unwrap();
                    let width = width.parse::<usize>().unwrap();
                    let height = height.parse::<usize>().unwrap();
                    screen.rect(width, height);
                },
                "rotate row y=" => {
                    let (row_num, dist) = rest.split_once(" by ").unwrap();
                    let row_num = row_num.parse::<usize>().unwrap();
                    let dist = dist.parse::<usize>().unwrap();
                    screen.rotate_row(row_num, dist);
                },
                "rotate column x=" => {
                    let (col_num, dist) = rest.split_once(" by ").unwrap();
                    let col_num = col_num.parse::<usize>().unwrap();
                    let dist = dist.parse::<usize>().unwrap();
                    screen.rotate_col(col_num, dist);
                },
                _ => println!("Invalid input"),
            }
        }
    }
    println!("{}", screen);
    screen.count_lit()
}

fn main() {
    let input = include_str!("input8.txt");
    println!("{}", part1(input));
}