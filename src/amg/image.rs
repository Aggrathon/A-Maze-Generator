
use itertools::Itertools;
use image::{GrayImage, Luma, FilterType, imageops::resize, Rgb, RgbImage, ConvertBuffer};
use super::Maze;

impl Maze {
    pub fn to_image(&self, tile_width: u32) -> GrayImage {
        let mut img = GrayImage::from_fn(self.width as u32, self.height as u32, |x, y| {
            match self.get(x as usize, y as usize) {
                d if d < 0 => Luma([0u8]),
                0          => Luma([32u8]),
                d if d > 0 => Luma([255u8]),
                _          => Luma([128u8])
            }
        });
        self.structures.iter().for_each(|i| i.for_each(|(x, y)| img.put_pixel(x as u32, y as u32, Luma([192u8]))));
        resize(&img, img.width() * tile_width, img.height() * tile_width, FilterType::Nearest)
    }

    pub fn to_image_color(&self, tile_width: u32) -> RgbImage {
        self.to_image(tile_width).convert() as RgbImage
    }
}

pub fn add_path_to_maze_image(maze: &Maze, path: &[usize], img: &mut RgbImage, color: Rgb<u8>, offset: u32) {
    if path.len() == 0 { return; }
    let tile_width = img.width() / (maze.width as u32);
    let mut prev = path[0];
    for p in path.iter().skip(1) {
        let (x1, y1) = maze.index_to_coordinate(*p);
        let (x2, y2) = maze.index_to_coordinate(prev);
        let x3 = std::cmp::min(x1, x2) as u32 * tile_width + offset;
        let x4 = std::cmp::max(x1, x2) as u32 * tile_width + offset + 1;
        let y3 = std::cmp::min(y1, y2) as u32 * tile_width + offset;
        let y4 = std::cmp::max(y1, y2) as u32 * tile_width + offset + 1;
        (x3..x4).cartesian_product(y3..y4).for_each(|(x, y)| img.put_pixel(x, y, color));
        prev = *p;
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image() {
        let maze = Maze::new(5, 4, true);
        let img = maze.to_image(3);
        assert_eq!((maze.width * 3) as u32, img.width());
        assert_eq!(4 * 3, img.height());
    }

    #[test]
    fn test_image_path() {
        let maze = Maze::new(6, 5, true);
        let img = maze.to_image(3);
        let mut img2: RgbImage = img.convert();
        add_path_to_maze_image(&maze, &[9 as usize, 10, 16, 17, 23], &mut img2, Rgb([255u8, 128u8, 128u8]), 1);
        assert_eq!((maze.width * 3) as u32, img2.width());
        assert_eq!(5 * 3, img2.height());
    }
}
