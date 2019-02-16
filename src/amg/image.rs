
use std::vec::Vec;
use image::{ImageBuffer, Luma, FilterType, imageops::resize};
use super::Maze;

impl Maze {
    pub fn to_image(&self, tile_width: u32) -> ImageBuffer<Luma<u8>, Vec<u8>> {
        let mut img = ImageBuffer::from_fn(self.width as u32, self.height as u32, |x, y| {
            match self.get(x as usize, y as usize) {
                d if d < 0 => Luma([0u8]),
                0          => Luma([64u8]),
                d if d > 0 => Luma([255u8]),
                _          => Luma([128u8])
            }
        });
        self.structures.iter().for_each(|i| i.for_each(|(x, y)| img.put_pixel(x as u32, y as u32, Luma([192u8]))));
        resize(&img, img.width() * tile_width, img.height() * tile_width, FilterType::Nearest)
    }
}