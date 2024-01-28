use nom::{
    IResult,
    bytes::complete::tag,
    sequence::tuple,
    character::complete::digit1,
    combinator::{map_res, map, rest},
};

fn parse_integer(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse::<u32>)(input)
}

fn line_parser(input: &str) -> IResult<&str, (u32, u32, u32)> {
    map(tuple((tag("Disc #"), parse_integer, tag(" has "), parse_integer, tag(" positions; at time=0, it is at position "), parse_integer, rest)),|(_, depth, _, len, _, pos, _)| {
            (depth, len, pos)
        },
    )(input)
}

#[derive(Debug, Clone)]
struct Disks {
    disks: Vec<Disk>,
}

impl Disks {
    fn find_delay(&self) -> u32 {
        (0..)
        .find(|&delay| {
            self.disks.iter().all(|d| {
                (d.pos + delay + d.depth) % d.len == 0
            })
        })
        .unwrap()
    }
}

#[derive(Debug, Clone)]
struct Disk {
    depth: u32,
    len: u32,
    pos: u32,
}

impl Disk {
    fn new(depth: u32, len: u32, pos: u32) -> Self {
        Self { depth, len, pos }
    }
}

fn parse() -> Disks {
    let input = include_str!("input15.txt");
    let disks = input.lines().map(|l| {
        let (_, (depth, len, pos)) = line_parser(l).unwrap();
        Disk::new(depth, len, pos)
    }).collect();
    let disks = Disks { disks };
    disks
}

fn part1() -> u32 {
    let disks = parse();
    disks.find_delay()
}

fn part2() -> u32 {
    let mut disks = parse();
    disks.disks.push(Disk::new(7, 11, 0));
    disks.find_delay()
}

fn main() {
    println!("{}", part2());
}