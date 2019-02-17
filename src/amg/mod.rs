
use rand::thread_rng;
use rand::seq::SliceRandom;

pub mod structures;
pub mod wilson;
pub mod kruskal;
pub mod image;
pub mod solve;
mod utils;

pub struct Maze {
    pub maze: Vec<i32>,
    pub width: usize,
    pub height: usize,
    pub structures: Vec<structures::Rect>,
    counter: i32
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Maze {
        let mut maze: Vec<i32> = vec![0; width*height];
        for i in 0..width {
            maze[i] = -1;
            maze[width * height - i - 1] = -1;
        }
        for i in 1..(height-1) {
            maze[i*width] = -1;
            maze[(i+1)*width - 1] = -1;
        }
        let x = width / 2;
        maze[x] = 1;
        maze[width*height - x - 1] = 2;
        Maze {maze, width, height, counter: 3, structures: vec![]}
    }

    pub fn generate(&mut self) {
        let mut rnd = thread_rng();
        let str_size = 4;
        let str_cnt = (self.width * self.height) / (str_size * str_size * 4);
        // Structures
        self.structures = structures::generate(self, str_cnt, str_size, str_size, 3, 4);
        let size = self.maze.len();
        let important_points: Vec<usize> = (0..size).filter(|x| self.maze[*x] > 0).collect();
        for i in important_points.iter() {
            wilson::carve_from_room(self, *i);
        }
        wilson::generate_sparse(self);
        kruskal::generate(self);
        // Remove stubs (randomly)
        let mut spaces: Vec<usize> = (self.width..(size-self.width)).filter(|x| self.maze[*x] == 1 && 
            vec![*x + 1, *x - 1, *x + self.width, *x - self.width].iter().filter(|y| self.maze[**y] > 0).count() == 1).collect();
        spaces.shuffle(&mut rnd);
        for i in spaces.iter() {
            let j = vec![*i + 1, *i - 1, *i + self.width, *i - self.width].iter().filter(|y| self.maze[**y] > 0).fold(*i, |_, x| *x);
            if vec![j + 1, j - 1, j + self.width, j - self.width].iter().filter(|y| self.maze[**y] > 0).count() > 2 {
                self.maze[*i] = 0;
            }
        }
    }

    pub fn get(&self, x:usize, y:usize) -> i32 {
        if x < self.width && y < self.height {
            self.maze[x + y * self.width]
        } else {
            0
        }
    }

    pub fn index_to_coordinate(&self, i:usize) -> (usize, usize) {
        return (i%self.width, i/self.width);
    }

    pub fn print(&self) {
        for i in (0..self.height).rev() {
            for j in 0..self.width {
                let tile = self.maze[j + i * self.width];
                if tile == 0 {
                    print!("##");
                } else if tile < 0 {
                    print!("@@");
                } else if tile == 1 {
                    print!("  ");
                } else {
                    print!("{} ", tile);
                }
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        let mut maze = Maze::new(5, 5);
        maze.generate();
        assert!(maze.maze.iter().filter(|x| **x > 0).count() > 4);
        assert_eq!(maze.get(2, 4), maze.get(2, 0));
        assert_eq!(maze.get(2, 4), 1);
    }

    #[test]
    fn test_index() {
        let mut maze = Maze::new(5, 5);
        maze.maze[13] = 3;
        assert_eq!(3, maze.index_to_coordinate(13).0);
        assert_eq!(2, maze.index_to_coordinate(13).1);
        assert_eq!(3, maze.get(3, 2));
    }
}