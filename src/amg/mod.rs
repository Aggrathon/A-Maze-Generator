
use rand::thread_rng;
use rand::seq::SliceRandom;

mod structures;

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
        let str_cnt = (self.width * self.height) / (str_size * str_size * 2);
        let doors: Vec<usize> = structures::add_structures(self, str_cnt, str_size, str_size, 3, 4);
        for i in doors.iter() {
            let x = *i%self.width;
            let y = *i/self.width;
            wilson(self, x, y);
        }
        loop {
            let mut tiles: Vec<usize> = (0..(self.width*self.height)).filter(|x| self.maze[*x] == 0).collect();
            tiles.shuffle(&mut rnd);
            for i in tiles.iter() {
                let neigh: Vec<i32> = vec![i-1, i+1, i-self.width, i+self.width].into_iter().map(|x| self.maze[x]).filter(|x| *x > 0).collect();
                if neigh.len() == 0 {
                    let x = *i%self.width;
                    let y = *i/self.width;
                    wilson(self, x, y);
                } else if neigh.len() > 1 && neigh.iter().any(|x| *x != neigh[1]) {
                    let x = *i%self.width;
                    let y = *i/self.width;
                    kruskal(self, x, y);
                }
            }
            if doors.iter().all(|x| self.maze[*x] == 1) {
                break;
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

pub fn wilson(maze: &mut Maze, x: usize, y: usize) {
    let index: usize = x + y * maze.width;
    if maze.maze.len() <= index || maze.maze[index] > 0 {
        return;
    } else {
        let mut walk: Vec<usize> = vec![index];
        // Random Walk
        // Remove Loops
        // Update maze
        return;
    }
}

pub fn kruskal(mase: &mut Maze, x: usize, y: usize) {

}


#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_wilson() {
    //     let mut maze = Maze::new(5, 5);
    //     wilson(&mut maze, 3, 3);
    //     assert_ne!(maze.get(3, 3), 0);
    // }
}