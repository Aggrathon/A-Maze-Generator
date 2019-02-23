extern crate rand;
extern crate itertools;
extern crate image;

use image::Rgb;

mod amg;

fn main() {
    let mut maze = amg::Maze::new(50, 50, true);
    maze.generate();
    maze.print();
    maze.to_image(3).save("maze.png").unwrap();
    let mut image = maze.to_image_color(6);
    for (i, c) in [
        Rgb([255, 128, 128]),
        Rgb([128, 128, 255]),
        Rgb([255, 64, 255]),
        Rgb([64, 255, 255]),
        Rgb([64, 255, 128]),
        Rgb([255, 255, 128])
    ].iter().enumerate() {
        let path = amg::solve::recursive_backtracker(&maze, maze.width/2, maze.maze.len()-maze.width/2-1);
        amg::image::add_path_to_maze_image(&maze, &path, &mut image, *c, i as u32);
    }
    image.save("maze_solved.png").unwrap();
}
