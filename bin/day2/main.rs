#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Coord {
    x: isize,
    y: isize,
}

fn is_valid_coordinate(coord: Coord) -> bool {
    let distance = coord.x.abs() + coord.y.abs();
    distance <= 2
}

fn part1(input: &str) -> u32 {
    let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let mut start = Coord { x: 1, y: 1 };

    input.lines().fold(0_u32, |acc, line| {
        line.trim().chars().for_each(|c| match c {
            'R' => if start.x < 2 { start.x += 1 },
            'L' => if start.x > 0 { start.x -= 1 },
            'U' => if start.y > 0 { start.y -= 1 },
            'D' => if start.y < 2 { start.y += 1 },
            _ => {},
        });
        let a = start.x as usize;
        let b = start.y as usize;
        acc * 10 + grid[b][a]
    })
}

fn part2(input: &str) -> String {
    let grid: Vec<Vec<&str>> = vec![
        vec!["0", "0", "1", "0", "0"],
        vec!["0", "2", "3", "4", "0"],
        vec!["5", "6", "7", "8", "9"],
        vec!["0", "A", "B", "C", "0"],
        vec!["0", "0", "D", "0", "0"],
    ];
    let mut start = Coord { x: -2, y: 0 };
    let mut answer = String::new();

    for line in input.lines() {
        for c in line.trim().chars() {
            let mut next = start;
            match c {
                'R' => next.x += 1,
                'L' => next.x -= 1,
                'U' => next.y -= 1,
                'D' => next.y += 1,
                _ => {},
            }
            if is_valid_coordinate(next) {
                start = next;
            }
        }
        let a = (start.x + 2) as usize;
        let b = (start.y + 2) as usize;
        answer.push_str(grid[b][a]);
    }
    answer
}
fn main() {
    let input = include_str!("input2.txt");
    println!("{}, {}", part1(input), part2(input));
}