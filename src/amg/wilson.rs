use super::Maze;
use super::kruskal;

use rand::thread_rng;
use rand::seq::SliceRandom;

pub fn random_walk(maze: &mut Maze, index: usize) {
    let i = index as i32;
    let mut rng = thread_rng();
    if maze.maze.len() <= index || maze.maze[index] > 0 {
        return;
    } else {
        let mut walk: Vec<usize> = vec![index];
        let mut dirs: Vec<i32> = vec![-1, 1, -(maze.width as i32), maze.width as i32];
        // Random Walk
        'outer: loop {
            dirs.shuffle(&mut rng);
            for d in dirs.iter() {
                let di = (*d + i) as usize;
                if maze.maze[di] > 0 && maze.maze[*walk.last().unwrap()] != maze.maze[di] {
                    walk.push(di);
                    break 'outer;
                }
                if maze.maze[di] == 0 {
                    walk.push(di);
                    continue 'outer;
                }
            }
            break;
        }
        // TODO: Remove Loops
        // Update maze
        dirs.push(0);
        let set = dirs.iter().map(|x| (x + i) as usize)
            .chain(dirs.iter().map(|x| (x + *walk.last().unwrap() as i32) as usize))
            .fold(maze.counter, |x, di| if maze.maze[di] > 0 { maze.maze[di] } else { x });
        for w in walk.iter() {
            maze.maze[*w] = set;
        }
        maze.counter += 1;
        kruskal::set_join(maze, index);
        kruskal::set_join(maze, *walk.last().unwrap());
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wilson() {
        let mut maze = Maze::new(5, 5);
        random_walk(&mut maze, 3 * 5 + 3);
        assert_ne!(maze.get(3, 3), 0);
    }
}
