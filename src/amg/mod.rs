
pub mod structures;
pub mod wilson;
pub mod kruskal;
pub mod image;
pub mod solve;
pub mod clean;
pub mod builder;
mod utils;

pub struct Maze {
    pub maze: Vec<i32>,
    pub width: usize,
    pub height: usize,
    pub structures: Vec<structures::Rect>,
    counter: i32
}

impl Maze {
    pub fn new(width: usize, height: usize, exits: bool) -> Maze {
        let mut maze: Vec<i32> = vec![0; width*height];
        for i in 0..width {
            maze[i] = -1;
            maze[width * height - i - 1] = -1;
        }
        for i in 1..(height-1) {
            maze[i*width] = -1;
            maze[(i+1)*width - 1] = -1;
        }
        let counter;
        if exits {
            let x = width / 2;
            maze[x] = 1;
            maze[width*height - x - 1] = 2;
            counter = 3;
        } else {
            counter = 1;
        }
        Maze {maze, width, height, counter: counter, structures: vec![]}
    }

    pub fn generate(&mut self, loops: bool, ) {
        (0..self.maze.len()).filter(|i| self.maze[*i] > 0).collect::<Vec<usize>>()
            .into_iter().for_each(|i| wilson::carve_from_room(self, i, loops));
        //wilson::generate_sparse(self);
        kruskal::generate(self);
    }

    pub fn get(&self, x:usize, y:usize) -> i32 {
        if x < self.width && y < self.height {
            self.maze[x + y * self.width]
        } else {
            0
        }
    }

    pub fn set(&mut self, x:usize, y:usize, value:i32) {
        if x < self.width && y < self.height {
            self.maze[x + y * self.width] = value;
        }
    }

    pub fn index_to_coordinate(&self, i:usize) -> (usize, usize) {
        return (i%self.width, i/self.width);
    }

    pub fn coordinate_to_index(&self, x:usize, y:usize) -> usize {
        return x + self.width * y;
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
        let size = 13;
        let mut maze = Maze::new(size, size, true);
        maze.generate(true);
        assert!(maze.maze.iter().filter(|x| **x > 0).count() > 10);
        assert_eq!(maze.get(size/2, size-1), maze.get(size/2, 0));
        assert_eq!(maze.get(size/2, size-1), 1);
        maze.structures.iter().for_each(|s| {
            (*s).border(maze.width, true, |x| {
                if maze.maze[x] == 1 {
                    assert!(utils::get_num_neighbours(&maze.maze, x, maze.width) > 1)
                } else {
                    assert_eq!(maze.maze[x], -1);
                }
            });
        });
    }

    #[test]
    fn test_index() {
        let mut maze = Maze::new(5, 5, true);
        maze.maze[13] = 3;
        assert_eq!(3, maze.get(3, 2));
        assert_eq!(3, maze.index_to_coordinate(13).0);
        assert_eq!(2, maze.index_to_coordinate(13).1);
        assert_eq!(13, maze.coordinate_to_index(3, 2));
    }
}