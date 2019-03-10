
use super::Maze;
use image::{Rgb, RgbImage};
use itertools::Itertools;
use rand::thread_rng;
use rand::seq::SliceRandom;

pub fn recursive_backtracker(maze: &Maze, start: usize, end:usize) -> Vec<usize> {
    let mut cover: Vec<bool> = maze.maze.iter().map(|x| *x > 0).collect();
    let size = cover.len();
    let mut walk: Vec<usize> = vec![start];
    let mut dirs: Vec<i32> = vec![-1, 1, -(maze.width as i32), maze.width as i32];
    let mut rng = thread_rng();
    'outer: loop {
        dirs.shuffle(&mut rng);
        let pos = *walk.last().unwrap() as i32;
        for d in dirs.iter().map(|i| *i + pos).filter(|i| *i >= 0 && *i < size as i32) {
            let du = d as usize;
            if cover[du] {
                walk.push(du);
                if du == end {
                    return walk;
                }
                cover[du] = false;
                continue 'outer;
            }
        }
        walk.pop();
        if walk.len() == 0 {
            return vec![];
        }
    }  
}

fn add_path_to_maze_image(maze: &Maze, path: &[usize], img: &mut RgbImage, color: Rgb<u8>, offset: u32) {
    if path.len() == 0 { return; }
    let tile_width = img.width() / (maze.width as u32);
    let mut prev = path[0];
    for p in path.iter().skip(1) {
        let (x1, y1) = maze.index_to_coordinate(*p);
        let (x2, y2) = maze.index_to_coordinate(prev);
        let iter1 = (x1 as u32 * tile_width + offset)..(x2 as u32 * tile_width + offset + 1);
        let iter2 = (y1 as u32 * tile_width + offset)..(y2 as u32 * tile_width + offset + 1);
        iter1.cartesian_product(iter2).for_each(|(x, y)| img.put_pixel(x, y, color));
        prev = *p;
    }
}

pub fn draw_paths(maze: &Maze, starts: &Vec<usize>, ends: &Vec<usize>) -> RgbImage {
    if starts.len() != ends.len() {
        panic!("Starts and Ends must match");
    }
    let colors = [
        Rgb([31,120,180]),
        Rgb([51,160,44]),
        Rgb([227,26,28]),
        Rgb([255,127,0]),
        Rgb([106,61,154]),
        Rgb([255,255,153]),
        Rgb([177,89,40]),
        Rgb([166,206,227]),
        Rgb([178,223,138]),
        Rgb([251,154,153]),
        Rgb([253,191,111]),
        Rgb([202,178,214])
    ];
    if colors.len() < starts.len() {
        panic!("Cannot draw that many paths (not enough distinct colors)");
    }
    let mut image = maze.to_image_color(starts.len() as u32);
    for (i, ((c, s), e)) in colors.iter().zip(starts).zip(ends).enumerate() {
        let path = recursive_backtracker(&maze, *s, *e);
        add_path_to_maze_image(&maze, &path, &mut image, *c, i as u32);
    }
    image
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recursive() {
        let maze = Maze::builder(15, 20).build();
        assert_eq!(maze.maze[7], 1);
        assert_eq!(maze.maze[15*20-8], 1);
        let path = recursive_backtracker(&maze, 7, 15*20-8);
        let mut prev = maze.index_to_coordinate(path[0]);
        for p in path.iter().skip(1) {
            let next = maze.index_to_coordinate(*p);
            assert!((prev.0 as i32 - next.0 as i32).abs() + (prev.1 as i32 - next.1 as i32).abs() <= 1);
            prev = next;
        }
        assert_eq!(path[0], 7);
        assert_eq!(*path.last().unwrap(), 15*20-8);
    }

    #[test]
    fn test_image_path() {
        let maze = Maze::new(6, 5, true);
        let mut img = maze.to_image_color(3);
        add_path_to_maze_image(&maze, &[9 as usize, 10, 16, 17, 23], &mut img, Rgb([255u8, 128u8, 128u8]), 1);
        assert_eq!((maze.width * 3) as u32, img.width());
        assert_eq!(5 * 3, img.height());
    }
}
