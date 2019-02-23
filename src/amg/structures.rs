use super::Maze;
use rand::{Rng, seq::IteratorRandom};
use itertools::Itertools;

pub struct Rect(usize, usize, usize, usize);

impl Rect {
    fn overlaps(&self, other: &Rect) -> bool {
        !(self.2 < other.0 || self.0 > other.2 || self.3 < other.1 || self.1 > other.3)
    }

    fn border<P>(&self, width: usize, corners: bool, f: P) where P: FnMut(usize) {
        let t;
        let b;
        if corners {
            t = (self.0 - 1 + (self.1 - 1) * width)..(self.2 + 1 + (self.1 - 1) * width);
            b = (self.0 - 1 + (self.3) * width)..(self.2 + 1 + (self.3) * width);
        } else {
            t = (self.0 + (self.1 - 1) * width)..(self.2 + (self.1 - 1) * width);
            b = (self.0 + (self.3) * width)..(self.2 + (self.3) * width);
        }
        let l = (self.1..self.3).map(|x| x * width + self.0 - 1);
        let r = (self.1..self.3).map(|x| x * width + self.2);
        t.chain(b).chain(l).chain(r).for_each(f);
    }

    pub fn for_each<P>(&self, f: P) where P: FnMut((usize, usize)) {
        (self.0..self.2).cartesian_product(self.1..self.3).for_each(f);
    }
}

pub fn generate(maze: &mut Maze, count: usize, max_width: usize, max_height: usize, wall_offset: usize, max_doors: usize) -> Vec<Rect> {
    let mut rnd = rand::thread_rng();
    let mut struc: Vec<Rect> = Vec::new();
    let mut walls: Vec<usize> = Vec::new();
    'outer: for _ in 0..count {
        // Generate Room
        let w = rnd.gen_range(2, max_width+1);
        let h = rnd.gen_range(2, max_height+1);
        if w + wall_offset*2 >= maze.width || h + wall_offset*2 >= maze.height { continue; }
        let x = rnd.gen_range(wall_offset, maze.width - w - wall_offset - 1);
        let y = rnd.gen_range(wall_offset, maze.height - h - wall_offset - 1);
        let r = Rect(x, y, x + w, y + h);
        let r2 = Rect(x-wall_offset, y-wall_offset, x + w + wall_offset, y + h + wall_offset);
        // Check Collisions
        for r3 in struc.iter() { if r2.overlaps(&r3) { continue 'outer; }}
        // Create Room
        for k in y..(y + h) {
            for j in x..(x + w) {
                maze.maze[j + k * maze.width] = maze.counter;
            }
        }
        // Create Walls & Doors
        r.border(maze.width, true, |x| maze.maze[x] = -1);
        let nd = rnd.gen_range(2, max_doors);
        walls.clear();
        r.border(maze.width, false, |x| walls.push(x));
        walls.iter().choose_multiple(&mut rnd, nd).into_iter().for_each(|x| maze.maze[*x] = maze.counter);
        struc.push(r);
        maze.counter = maze.counter + 1
    }
    struc
}

pub fn generate_default(maze: &mut Maze) -> Vec<Rect> {
    let str_size = 4;
    let str_cnt = (maze.width * maze.height) / (str_size * str_size * 4);
    generate(maze, str_cnt, str_size, str_size, 3, str_size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect() {
        let r1 = Rect(1, 2, 4, 5);
        let r2 = Rect(2, 3, 3, 4);
        let r3 = Rect(2, 6, 3, 7);
        assert!(r1.overlaps(&r2));
        assert!(!r1.overlaps(&r3));
    }
}
