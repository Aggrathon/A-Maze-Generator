extern crate rand;

mod amg;

fn main() {
    let mut maze = amg::Maze::new(50, 30);
    maze.generate();
    maze.print();
}
