use image::{RgbImage, ImageBuffer, Rgb};

use crate::operations::pixel_average;

pub fn compensate(frame_1_filename: &str, frame_2_filename: &str, flow: &Vec<Vec<(f32, f32)>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let frame_1 = image::open(frame_1_filename).unwrap().into_rgb8();
    let frame_2 = image::open(frame_2_filename).unwrap().into_rgb8();
    let (width, height) = frame_1.dimensions();
    let mut img: RgbImage = ImageBuffer::new(width, height);
    
    for i in 0..height {
        for j in 0..width { 
            let (mut fdx, mut fdy) = flow[i as usize][j as usize];
            fdx /= 2.0; fdy /= 2.0;
            let dx = fdx.round() as i32;
            let dy = fdy.round() as i32;

            let res_1_x = (j as i32 + dx) as u32;
            let res_1_y = (i as i32 + dy) as u32;
            if res_1_x < width && res_1_y < height {
                if img.get_pixel(res_1_x, res_1_y) == &Rgb([0, 0, 0]) {
                    img.put_pixel(res_1_x, res_1_y, *frame_1.get_pixel(j, i));
                } else {
                    img.put_pixel(
                        res_1_x, 
                        res_1_y, 
                        pixel_average(
                            *img.get_pixel(res_1_x, res_1_y), 
                            *frame_2.get_pixel(j, i)
                        )
                    );
                }
            }
            let res_2_x = (j as i32 - dx) as u32;
            let res_2_y = (i as i32 - dy) as u32;
            if res_2_x < width && res_2_y < height {
                if img.get_pixel(res_2_x, res_2_y) == &Rgb([0, 0, 0]) {
                    img.put_pixel(res_2_x, res_2_y, *frame_2.get_pixel(j, i));
                } else {
                    img.put_pixel(
                        res_2_x, 
                        res_2_y, 
                        pixel_average(
                            *img.get_pixel(res_2_x, res_2_y), 
                            *frame_2.get_pixel(j, i)
                        )
                    );
                }
            }
        }
    }

    for i in 0..height {
        for j in 0..width { 
            if img.get_pixel(j, i) == &Rgb([0, 0, 0]) {
                img.put_pixel(
                    j, 
                    i, 
                    pixel_average(
                        *frame_1.get_pixel(j, i), 
                        *frame_2.get_pixel(j, i)
                    )
                );
            }
        }
    }

    img
}