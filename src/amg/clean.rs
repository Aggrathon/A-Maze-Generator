use super::utils;
use super::Maze;
use rand::seq::SliceRandom;
use rand::thread_rng;

impl Maze {
    pub fn remove_stubs(self: &mut Maze) {
        let size = self.maze.len();
        let mut spaces: Vec<usize> = ((self.width + 1)..(size - self.width - 1))
            .filter(|x| {
                self.maze[*x] > 0 && utils::get_num_neighbours(&self.maze, *x, self.width) == 1
            })
            .collect();
        spaces.shuffle(&mut thread_rng());
        for i in spaces.iter() {
            if self.maze[*i + 1] > 0 {
                if utils::get_num_neighbours(&self.maze, *i + 1, self.width) > 2 {
                    self.maze[*i] = 0;
                }
            } else if self.maze[*i - 1] > 0 {
                if utils::get_num_neighbours(&self.maze, *i - 1, self.width) > 2 {
                    self.maze[*i] = 0;
                }
            } else if self.maze[*i + self.width] > 0 {
                if utils::get_num_neighbours(&self.maze, *i + self.width, self.width) > 2 {
                    self.maze[*i] = 0;
                }
            } else if self.maze[*i - self.width] > 0
                && utils::get_num_neighbours(&self.maze, *i - self.width, self.width) > 2
            {
                self.maze[*i] = 0;
            }
        }
    }
    #[allow(dead_code)]
    pub fn remove_holes(self: &mut Maze) {
        self.maze
            .iter()
            .enumerate()
            .filter(|(i, x)| **x > 1 && utils::get_num_neighbours(&self.maze, *i, self.width) == 0)
            .map(|(i, _)| i)
            .collect::<Vec<usize>>()
            .iter()
            .for_each(|x| self.maze[*x] = 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rem_holes() {
        let mut maze = Maze::builder(20, 20).stubs(false).build();
        maze.remove_holes();
        assert_eq!(maze.maze.iter().filter(|x| **x > 1).count(), 0);
    }
}
