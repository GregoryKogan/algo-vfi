use image::{GenericImage, ImageBuffer, Pixel, Rgb, RgbImage};
use num::integer::Roots;

pub fn add_padding(
    img: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    p_width: u32,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    let mut out_img: RgbImage = ImageBuffer::new(width + p_width * 2, height + p_width * 2);
    out_img.copy_from(img, p_width, p_width).unwrap();
    for y in p_width..p_width + height {
        let l_pixel_val = out_img[(p_width, y)];
        let r_pixel_val = out_img[(p_width + width - 1, y)];
        for x in 0..p_width {
            out_img.put_pixel(x, y, l_pixel_val);
            out_img.put_pixel(x + width + p_width, y, r_pixel_val);
        }
    }
    for x in 0..width + 2 * p_width {
        let t_pixel_val = out_img[(x, p_width)];
        let b_pixel_val = out_img[(x, p_width + height - 1)];
        for y in 0..p_width {
            out_img.put_pixel(x, y, t_pixel_val);
            out_img.put_pixel(x, y + p_width + height, b_pixel_val);
        }
    }

    return out_img;
}

pub fn pixel_difference(pix_1: Rgb<u8>, pix_2: Rgb<u8>) -> u16 {
    let ch1 = pix_1.channels();
    let ch2 = pix_2.channels();
    return ((ch1[0].abs_diff(ch2[0]) as u32).pow(2)
        + (ch1[1].abs_diff(ch2[1]) as u32).pow(2)
        + (ch1[2].abs_diff(ch2[2]) as u32).pow(2))
    .sqrt() as u16;
}
