extern crate rand;
extern crate itertools;

mod amg;

fn main() {
    let mut maze = amg::Maze::new(50, 30);
    maze.generate();
    maze.print();
}
