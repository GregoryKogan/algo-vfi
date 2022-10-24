use image::{ImageBuffer, Rgb, RgbImage, Pixel};

pub fn get_conv_edges(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let (width, height) = img.dimensions();
    let mut res_img: RgbImage = ImageBuffer::new(width, height);

    let filter_1 = [
        [-1, 0, 1],
        [-1, 0, 1],
        [-1, 0, 1],
    ];

    let filter_2 = [
        [-1, -1, -1],
        [ 0,  0,  0],
        [ 1,  1,  1],
    ];

    for i in 1..height - 1 {
        for j in 1..width - 1 {
            let mut res_val: i32 = 0;
            for fi in 0..3 {
                for fj in 0..3 {
                    res_val += filter_1[fi][fj] * img[((j as i32 + fj as i32 - 1) as u32, (i as i32  + fi as i32 - 1) as u32)].channels()[0] as i32;
                    res_val += filter_2[fi][fj] * img[((j as i32 + fj as i32 - 1) as u32, (i as i32  + fi as i32 - 1) as u32)].channels()[0] as i32;
                }
            }
            res_val = res_val.clamp(0, 255);
            res_img.put_pixel(j, i, Rgb([res_val as u8, res_val as u8, res_val as u8]));
        }
    }

    for i in 0..height {
        for j in 0..width {
            img.put_pixel(j, i, res_img[(j, i)]);
        }
    }
}