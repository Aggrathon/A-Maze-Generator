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
        let mut cover: Vec<i32> = maze.maze.iter().map(|x| if *x == 0 { 0 } else { -1 }).collect();
        let mut walk: Vec<usize> = vec![index];
        let mut counter: i32 = 2;
        cover[index] = 1;
        let mut dirs: Vec<i32> = vec![-1, 1, -(maze.width as i32), maze.width as i32];
        // Random Walk
        'outer: loop {
            dirs.shuffle(&mut rng);
            for d in dirs.iter() {
                let di = (*d + *walk.last().unwrap() as i32) as usize;
                if maze.maze[di] > 0 && maze.maze[*walk.last().unwrap()] != maze.maze[di] {
                    break 'outer; // Found existing part of the maze, random walk is done
                }
                if cover[di] == 0 && maze.maze[di] == 0 {
                    walk.push(di);
                    cover[di] = counter;
                    counter += 1;
                    continue 'outer;
                }
            }
            walk.pop(); // Backtrack if dead end
            if walk.len() == 0 { return; }
        }
        // Reconstruct the path
        let mut end: usize = *walk.last().unwrap();
        let set = dirs.iter().map(|x| (x + i) as usize)
            .chain(dirs.iter().map(|x| (x + end as i32) as usize))
            .fold(if maze.maze[index] > 0 { maze.maze[index] } else { maze.counter }, |x, di| if maze.maze[di] > 0 { maze.maze[di] } else { x });
        maze.maze[index] = set;
        while cover[end] > 1 {
            maze.maze[end] = set;
            end = dirs.iter().fold(end, |x, di| {
                let d = (di + end as i32) as usize;
                if cover[d] > 0 && cover[d] < cover[x] { d } else { x }
            });
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
