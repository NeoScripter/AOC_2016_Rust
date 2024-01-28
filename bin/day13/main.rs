use std::collections::{HashSet, BinaryHeap};
use std::cmp::Reverse;
use std::fmt;

#[derive(Debug)]
struct Maze {
    maze: Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.maze {
            for num in row {
                write!(f, "{}", num)?;
            } 
            writeln!(f)?;
        }
        Ok(())
    }
}
impl Maze {
    fn new(start: (usize, usize), end: (usize, usize)) -> Self {
        Maze {
            maze: vec![vec!['#'; 300]; 300],
            start,
            end,
        }
    }
    fn find_walls(&mut self, num: usize) {
        (0..self.maze.len()).for_each(|y| {
            (0..self.maze[0].len()).for_each(|x| {
                let ones = (x*x + 3*x + 2*x*y + y + y*y + num).count_ones();
                if ones % 2 == 0 {self.maze[y][x] = '.'}
            })
        })
    }
    fn find_path(&self) -> u32 {
        let mut cache = HashSet::new();
        let mut queue = BinaryHeap::new();
        queue.push((Reverse(0), self.start));
        while let Some((Reverse(steps), (y, x))) = queue.pop() {
            if !cache.insert((y, x)) {continue}
            if steps == 50 {continue}
            // comment out the upper line and uncomment the lower line for part 1
            //if (y, x) == self.end {return steps}

            if x > 0 {if self.maze[y][x - 1] != '#' {queue.push((Reverse(steps + 1), (y, x - 1)))}}
            if x < self.maze[0].len() - 1 {if self.maze[y][x + 1] != '#' {queue.push((Reverse(steps + 1), (y, x + 1)))}}
            if y > 0 {if self.maze[y - 1][x] != '#' {queue.push((Reverse(steps + 1), (y - 1, x)))}}
            if y < self.maze.len() - 1 {if self.maze[y + 1][x] != '#' {queue.push((Reverse(steps + 1), (y + 1, x)))}}
        }
        // replace the following line with 0 for part 1
        cache.len() as u32
    }

}

fn solve(input: usize) -> u32 {
    let mut maze = Maze::new((1, 1), (39, 31));
    maze.find_walls(input);
    //println!("{}", maze);
    maze.find_path()
}
fn main() {
    println!("{}", solve(1352));
}