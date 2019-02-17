use super::Maze;
use super::kruskal;

use rand::thread_rng;
use rand::seq::SliceRandom;

// This is a implementation of the Wilson (random walk) maze algorithm that has been
// modified to include backtacking and handle already existing rooms
pub fn random_walk(maze: &mut Maze, index: usize) {
    let i = index as i32;
    let mut rng = thread_rng();
    if index < maze.maze.len() {
        let mut cover: Vec<i32> = maze.maze.iter().map(|x| if *x == 0 { 0 } else { -1 }).collect();
        let mut walk: Vec<usize> = vec![index];
        let mut counter: i32 = 2;
        cover[index] = 1;
        let mut dirs: Vec<i32> = vec![-1, 1, -(maze.width as i32), maze.width as i32];
        // If Door, then don't walk into the room
        if maze.maze[index] > 0 {
            dirs.shuffle(&mut rng);
            match dirs.iter().map(|x| (i + *x) as usize).filter(|x| maze.maze[*x] == 0).last() {
                Some(x) => {
                    cover[x] = counter;
                    counter += 1;
                    walk.push(x);
                    maze.maze[index] = 0;
                },
                None => {
                    return();
                }
            }
        }
        // Random Walk
        'outer: loop {
            dirs.shuffle(&mut rng);
            let pos = *walk.last().unwrap() as i32;
            if dirs.iter().map(|x| (pos + *x) as usize).filter(|x| maze.maze[*x] > 0).count() > 0 {
                break 'outer; // Found existing part of the maze, random walk is done
            }
            for d in dirs.iter() {
                let di = (*d + pos) as usize;
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
