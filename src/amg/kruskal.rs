
use super::Maze;
use super::utils;

use rand::thread_rng;
use rand::seq::SliceRandom;

pub fn set_join(maze: &mut Maze, index: usize) {
    let set: i32 = utils::get_lowest_neighbour(&maze.maze, index, maze.width, maze.counter);
    if set == maze.counter { maze.counter += 1; }
    let mut stack = vec![index];
    maze.maze[index] = set;
    while !stack.is_empty() {
        let i = stack.pop().unwrap();
        let x = i%maze.width;
        let y = i/maze.width;
        if x+1 < maze.width && maze.maze[i+1] > set { stack.push(i+1); maze.maze[i+1] = set; }
        if x > 0 && maze.maze[i-1] > set { stack.push(i-1); maze.maze[i-1] = set; }
        if y+1 < maze.height && maze.maze[i+maze.width] > set { stack.push(i+maze.width); maze.maze[i+maze.width] = set; }
        if y > 0 && maze.maze[i-maze.width] > set { stack.push(i-maze.width); maze.maze[i-maze.width] = set; }
    }
}

pub fn generate(maze: &mut Maze) {
    let mut rnd = thread_rng();
    let mut tiles: Vec<usize> = (0..maze.maze.len()).filter(|x| maze.maze[*x] == 0).collect();
    tiles.shuffle(&mut rnd);
    for i in tiles.iter() {
        match utils::get_num_diff_neighbours(&maze.maze, *i, maze.width) {
            utils::DiffNeigh::None => { maze.maze[*i] = maze.counter; maze.counter += 1; },
            utils::DiffNeigh::One => { set_join(maze, *i); },
            utils::DiffNeigh::MultDiff => { set_join(maze, *i); },
            _ => {}
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kruskal() {
        let mut maze = Maze::new(5, 5);
        generate(&mut maze);
        assert_eq!(maze.get(2, 4), maze.get(2, 0));
        assert_eq!(maze.get(2, 4), 1);
    }
}