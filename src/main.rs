extern crate rand;

mod amg;

fn main() {
    let mut maze = amg::Maze::new(20, 12);
    maze.generate();
    maze.print();
}
