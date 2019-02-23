
use super::Maze;
use super::utils;
use rand::thread_rng;
use rand::seq::SliceRandom;


pub fn remove_stubs(maze: &mut Maze) {
    let size = maze.maze.len();
    let mut spaces: Vec<usize> = ((maze.width+1)..(size-maze.width-1)).filter(|x| maze.maze[*x] > 0 && utils::get_num_neighbours(&maze.maze, *x, maze.width) == 1).collect();
    spaces.shuffle(&mut thread_rng());
    for i in spaces.iter() {
        if maze.maze[*i + 1] > 0 { if utils::get_num_neighbours(&maze.maze, *i + 1, maze.width) > 2 { maze.maze[*i] = 0; } }
        else if maze.maze[*i - 1] > 0 { if utils::get_num_neighbours(&maze.maze, *i - 1, maze.width) > 2 { maze.maze[*i] = 0; } }
        else if maze.maze[*i + maze.width] > 0 { if utils::get_num_neighbours(&maze.maze, *i + maze.width, maze.width) > 2 { maze.maze[*i] = 0; } }
        else if maze.maze[*i - maze.width] > 0 { if utils::get_num_neighbours(&maze.maze, *i - maze.width, maze.width) > 2 { maze.maze[*i] = 0; } }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recursive() {
        let mut maze = Maze::new(5, 5, true);
        maze.set(3,3,1);
        maze.set(3,4,1);
        maze.set(3,2,1);
        maze.set(4,3,1);
        let num = maze.maze.iter().filter(|x| **x > 0).count();
        remove_stubs(&mut maze);
        let num2 = maze.maze.iter().filter(|x| **x > 0).count();
        assert_eq!(num-1, num2);
    }
}
