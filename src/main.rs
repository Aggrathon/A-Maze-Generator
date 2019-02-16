extern crate rand;
extern crate itertools;
extern crate image;

mod amg;

fn main() {
    let mut maze = amg::Maze::new(50, 50);
    maze.generate();
    maze.print();
    maze.to_image(3).save("maze.png").unwrap();
}
