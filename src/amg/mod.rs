
use rand::thread_rng;
use rand::seq::SliceRandom;
use itertools::Itertools;

mod structures;
mod wilson;
mod kruskal;

pub struct Maze {
    pub maze: Vec<i32>,
    pub width: usize,
    pub height: usize,
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
        Maze {maze, width, height, counter: 3}
    }

    pub fn generate(&mut self) {
        let mut rnd = thread_rng();
        let str_size = 3;
        let str_cnt = (self.width * self.height) / (str_size * str_size * 4);
        // Structures
        structures::generate(self, str_cnt, str_size, str_size, 3, 4);
        let size = self.maze.len();
        let important_points: Vec<usize> = (0..size).filter(|x| self.maze[*x] > 0).collect();
        for i in important_points.iter() {
            if *i >= self.width && *i < size - self.width {
                wilson::random_walk(self, *i);
            }
        }
        // Wilson Random Walk
        let dirs: Vec<i32> = vec![-1, 1, -(self.width as i32), self.width as i32];
        let mut tiles: Vec<usize> = (0..size).filter(|x| self.maze[*x] == 0).collect();
        tiles.shuffle(&mut rnd);
        for i in tiles.iter() {
            if self.maze[*i] != 0 { continue; }
            if dirs.iter().map(|x| self.maze[(*i as i32 + x) as usize]).filter(|x| *x > 0).count() == 0 {
                wilson::random_walk(self, *i);
            }
        }
        // Kruskal Set Join
        loop {
            let mut tiles: Vec<usize> = (0..size).filter(|x| self.maze[*x] == 0).collect();
            tiles.shuffle(&mut rnd);
            for i in tiles.iter() {
                if self.maze[*i] != 0 { continue; }
                let neigh: Vec<i32> = dirs.iter().map(|x| self.maze[(*i as i32 + x) as usize]).filter(|x| *x > 0).collect();
                if neigh.len() == 0 {
                    wilson::random_walk(self, *i);
                } else if neigh.len() == 1 || neigh.iter().unique().count() > 1 {
                    kruskal::set_join(self, *i);
                }
            }
            if important_points.iter().map(|x| self.maze[*x]).unique().count() == 1 {
                break;
            }
            break;
        }
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
