
use super::Maze;

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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recursive() {
        let mut maze = Maze::new(15, 20, true);
        maze.generate();
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
}
