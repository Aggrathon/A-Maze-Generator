
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
}
