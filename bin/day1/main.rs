use std::collections::HashSet;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Coord(i32, i32);

fn main() {
    let mut moves: HashSet<Coord> = HashSet::new();
    let mut coord = Coord(0, 0);
    let mut degree: i32 = 0;
    let input = include_str!("input1.txt");
    'outer: for comd in input.split(", ") {
        let (dir, blocks_str) = comd.split_at(1);
        let blocks: i32 = blocks_str.parse().expect("Failed to parse the number");
        degree += match dir {
            "R" => 90,
            "L" => -90,
            _ => panic!("Invalid input!"),
        };
        let compass = degree.rem_euclid(360);
        for _ in 1..=blocks {
            match compass {
                0 => coord.1 += 1,
                90 => coord.0 += 1,
                180 => coord.1 -= 1,
                270 => coord.0 -= 1,
                _ => panic!("The number shouldn't exceed 360"),
            }
            if !moves.insert(coord) {
                println!("{}", coord.0.abs() + coord.1.abs());
                break 'outer;
            }
        }
    }
}
