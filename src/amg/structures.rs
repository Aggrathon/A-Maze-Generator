use super::Maze;
use itertools::Itertools;
use rand::{seq::IteratorRandom, Rng};

pub struct Rect(usize, usize, usize, usize);

impl Rect {
    pub fn overlaps(&self, other: &Rect) -> bool {
        !(self.2 < other.0 || self.0 > other.2 || self.3 < other.1 || self.1 > other.3)
    }

    pub fn border<P>(&self, width: usize, corners: bool, f: P)
    where
        P: FnMut(usize),
    {
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

    pub fn for_each<P>(&self, f: P)
    where
        P: FnMut((usize, usize)),
    {
        (self.0..self.2)
            .cartesian_product(self.1..self.3)
            .for_each(f);
    }
}

impl Maze {
    pub fn generate_structures(
        self: &mut Maze,
        count: usize,
        max_width: usize,
        max_height: usize,
        wall_offset: usize,
        max_doors: usize,
    ) {
        let mut rnd = rand::thread_rng();
        let mut struc: Vec<Rect> = Vec::new();
        let mut walls: Vec<usize> = Vec::new();
        'outer: for _ in 0..count {
            // Generate Room
            let w = rnd.gen_range(2, max_width + 1);
            let h = rnd.gen_range(2, max_height + 1);
            if w + wall_offset * 2 >= self.width || h + wall_offset * 2 >= self.height {
                continue;
            }
            let x = rnd.gen_range(wall_offset, self.width - w - wall_offset - 1);
            let y = rnd.gen_range(wall_offset, self.height - h - wall_offset - 1);
            let r = Rect(x, y, x + w, y + h);
            let r2 = Rect(
                x - wall_offset,
                y - wall_offset,
                x + w + wall_offset,
                y + h + wall_offset,
            );
            // Check Collisions
            for r3 in struc.iter() {
                if r2.overlaps(r3) {
                    continue 'outer;
                }
            }
            // Create Room
            for k in y..(y + h) {
                for j in x..(x + w) {
                    self.maze[j + k * self.width] = self.counter;
                }
            }
            // Create Walls & Doors
            r.border(self.width, true, |x| self.maze[x] = -1);
            let nd = rnd.gen_range(2, max_doors);
            walls.clear();
            r.border(self.width, false, |x| walls.push(x));
            walls
                .iter()
                .choose_multiple(&mut rnd, nd)
                .into_iter()
                .for_each(|x| self.maze[*x] = self.counter);
            struc.push(r);
            self.counter += 1;
        }
        self.structures = struc;
    }

    pub fn generate_structures_default(self: &mut Maze) {
        let str_size = 4;
        let str_cnt = (self.width * self.height) / (str_size * str_size * 4);
        self.generate_structures(str_cnt, str_size, str_size, 3, str_size);
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
