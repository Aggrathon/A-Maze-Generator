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
        let x = width / 2;
        maze[x] = 1;
        maze[width*height - x - 1] = 2;
        Maze {maze, width, height, counter: 3}
    }

    pub fn generate(&mut self) {
        let str_size = 3;
        let str_cnt = (self.width * self.height) / (str_size * str_size * 4);
        structures::add_structures(self, str_cnt, str_size, str_size, 3);
        // loop { //TODO: Check for stopping
        //     wilson(self, 0, 0);
        // }
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
                if tile <= 0 {
                    print!("# ");
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