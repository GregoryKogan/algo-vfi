use image::{ImageBuffer, Rgb, RgbImage};

use crate::plotter::draw_line;

pub fn visualize_as_vector_field(
    vf: Vec<Vec<(i16, i16)>>,
    block_size: u32,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (height, width) = (
        vf.len() as u32 * block_size,
        vf[0].len() as u32 * block_size,
    );
    let mut img: RgbImage = ImageBuffer::new(width, height);
    for i in 0..vf.len() {
        for j in 0..vf[i].len() {
            if vf[i][j] == (0, 0) {
                continue;
            }
            let sx = j as u32 * block_size + block_size / 2;
            let sy = i as u32 * block_size + block_size / 2;
            let ex = (sx as i32 + vf[i][j].0 as i32).clamp(0, (width - 1) as i32) as u32;
            let ey = (sy as i32 + vf[i][j].1 as i32).clamp(0, (height - 1) as i32) as u32;
            draw_line(&mut img, (sx, sy), (ex, ey), Rgb([0, 255, 0]), 1);
            img.put_pixel(sx, sy, Rgb([0, 0, 255]));
            img.put_pixel(ex, ey, Rgb([255, 0, 0]));
        }
    }
    img
}
