use super::Maze;
use super::kruskal;
use super::utils;

use rand::thread_rng;
use rand::seq::SliceRandom;

// This is a implementation of the Wilson (random walk) maze algorithm that has been
// modified to include backtacking and handle already existing rooms
pub fn random_walk(maze: &mut Maze, index: usize) -> Vec<usize> {
    let mut rng = thread_rng();
    if index >= maze.maze.len() { return vec![] }
    let mut cover: Vec<i32> = maze.maze.iter().map(|x| if *x == 0 { 0 } else { -1 }).collect();
    let mut walk: Vec<usize> = vec![index];
    let mut counter: i32 = 2;
    cover[index] = 1;
    let mut dirs: Vec<i32> = vec![-1, 1, -(maze.width as i32), maze.width as i32];
    // Random Walk
    'outer: loop {
        let pos = *walk.last().unwrap() as i32;
        if dirs.iter().map(|x| (pos + *x) as usize).filter(|x| *x < maze.maze.len() && maze.maze[*x] > 0).count() > 0 {
            break 'outer; // Found existing part of the maze, random walk is done
        }
        dirs.shuffle(&mut rng);
        for d in dirs.iter() {
            let di = (*d + pos) as usize;
            if di < maze.maze.len() && cover[di] == 0 && maze.maze[di] == 0 {
                walk.push(di);
                cover[di] = counter;
                counter += 1;
                continue 'outer;
            }
        }
        walk.pop(); // Backtrack if dead end
        if walk.len() == 0 { return vec![]; }
    }
    // Reconstruct the path
    let mut end: usize = *walk.last().unwrap();
    walk.clear();
    walk.push(end);
    while cover[end] > 1 {
        end = dirs.iter().fold(end, |x, di| {
            let d = (di + end as i32) as usize;
            if cover[d] > 0 && cover[d] < cover[x] { d } else { x }
        });
        walk.push(end);
    }
    walk
}

pub fn carve(maze: &mut Maze, index: usize) {
    if index >= maze.maze.len() || maze.maze[index] != 0 { return; }
    let path = random_walk(maze, index);
    if path.len() == 0 { return; }
    let end: usize = *path.first().unwrap();
    let set = utils::get_lowest_neighbour(&maze.maze, end, maze.width, maze.counter);
    path.into_iter().for_each(|i| maze.maze[i] = set);
}

pub fn carve_from_room(maze: &mut Maze, index: usize, loops: bool) {
    if index >= maze.maze.len() || maze.maze[index] <= 0 { return; }
    // Exit the room if possible
    let r = maze.maze[index];
    let start;
    match utils::get_neighbours_wrapping(index, maze.width).iter().filter(|x| **x < maze.maze.len() && maze.maze[**x] == 0).nth(0) {
        Some(x) => { maze.maze[index] = -1; start = *x; },
        None => { return; }
    }
    // Then do a random walk
    let path = random_walk(maze, start);
    let end: usize = *path.first().unwrap();
    let set = utils::get_lowest_neighbour(&maze.maze, end, maze.width, maze.counter);
    if r < set {
        maze.maze[index] = r;
        path.into_iter().for_each(|i| maze.maze[i] = r);
        kruskal::set_join(maze, end);
    } else if set < r {
        maze.maze[index] = set;
        kruskal::set_join(maze, index);
        path.into_iter().for_each(|i| maze.maze[i] = set);
    } else if loops {
        maze.maze[index] = set;
        path.into_iter().for_each(|i| maze.maze[i] = set);
    } else {
        let len = path.len() / 2;
        dbg!(len);
        maze.maze[index] = r;
        path.into_iter().skip(len).for_each(|i| maze.maze[i] = r);
    }
}

pub fn generate(maze: &mut Maze) {
    let size = maze.maze.len();
    if !maze.maze.iter().any(|x| *x > 0) { utils::dot_init_maze(maze) }
    (0..size).for_each(|x| {
        if maze.maze[x] == 0 {
            match utils::get_num_diff_neighbours(&maze.maze, x, maze.width) {
                utils::DiffNeigh::None => { carve(maze, x); },
                utils::DiffNeigh::One => { kruskal::set_join(maze, x); },
                utils::DiffNeigh::MultDiff => { kruskal::set_join(maze, x); },
                _ => {}
            }
        }
    });
}

pub fn generate_sparse(maze: &mut Maze) {
    let size = maze.maze.len();
    if !maze.maze.iter().any(|x| *x > 0) { utils::dot_init_maze(maze) }
    (0..size).for_each(|x| {
        if maze.maze[x] == 0 {
            match utils::get_num_diff_neighbours(&maze.maze, x, maze.width) {
                utils::DiffNeigh::None => { carve(maze, x); },
                utils::DiffNeigh::MultDiff => { kruskal::set_join(maze, x); },
                _ => {}
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wilson1() {
        let mut maze = Maze::new(5, 5, false);
        maze.set(2, 0, 0);
        maze.set(2, 4, 0);
        generate(&mut maze);
        assert!(maze.maze.iter().filter(|x| **x > 0).count() > 4);
        assert_eq!(maze.get(2, 0), 1);
        assert_eq!(1, maze.get(2, 4));
    }

    #[test]
    fn test_wilson2() {
        let mut maze = Maze::new(5, 5, true);
        generate_sparse(&mut maze);
        assert!(maze.maze.iter().filter(|x| **x > 0).count() > 4);
    }

    #[test]
    fn test_wilson3() {
        let mut maze = Maze::new(5, 5, true);
        let i = maze.coordinate_to_index(2, 0);
        carve_from_room(&mut maze, i, true);
        assert!(maze.maze.iter().filter(|x| **x > 0).count() > 4);
        assert_eq!(maze.get(2, 0), maze.get(2, 4));
    }

    #[test]
    fn test_wilson4() {
        let mut maze = Maze::new(3, 4, true);
        maze.set(1, 0, 1);
        maze.set(1, 3, 1);
        let i = maze.coordinate_to_index(1, 0);
        carve_from_room(&mut maze, i, false);
        assert_eq!(maze.maze.iter().filter(|x| **x > 0).count(), 3);
    }
}
