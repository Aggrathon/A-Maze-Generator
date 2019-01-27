use super::Maze;
use rand::Rng;

struct Rect(usize, usize, usize, usize);

impl Rect {
    fn overlaps(&self, other: &Rect) -> bool {
        !(self.2 < other.0 || self.0 > other.2 || self.3 < other.1 || self.1 > other.3)
    }
}

pub fn add_structures(maze: &mut Maze, count: usize, max_width: usize, max_height: usize, wall_offset: usize) {
    let mut rnd = rand::thread_rng();
    let mut struc: Vec<Rect> = Vec::new();
    'outer: for i in 0..count {
        let w = rnd.gen_range(2, max_width+1);
        let h = rnd.gen_range(2, max_height+1);
        if w + wall_offset*2 >= maze.width || h + wall_offset*2 >= maze.height { continue; }
        let x = rnd.gen_range(wall_offset, maze.width - w - wall_offset - 1);
        let y = rnd.gen_range(wall_offset, maze.height - h - wall_offset - 1);
        let r = Rect(x, y, x + w, y + h);
        let r2 = Rect(x-wall_offset, y-wall_offset, x + w + wall_offset, y + h + wall_offset);
        for r3 in struc.iter() { if r2.overlaps(&r3) { continue 'outer; }}
        struc.push(r);
        for k in y..(y+h) {
            for j in x..(x+w) {
                maze.maze[j + k * maze.width] = maze.counter;
            }
        }
        maze.counter = maze.counter + 1
    }
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
