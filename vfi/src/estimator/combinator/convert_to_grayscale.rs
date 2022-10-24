use image::{ImageBuffer, Rgb, Pixel};

pub fn to_grayscale(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let (width, height) = img.dimensions();
    for i in 0..height {
        for j in 0..width {
            let res_color = pixel_to_grayscale(img[(j, i)]);
            img.put_pixel(j, i, res_color);
        }
    }
}

fn pixel_to_grayscale(pix: Rgb<u8>) -> Rgb<u8> {
    let colors = pix.channels();
    let br = colors[0] / 3 + colors[1] / 3 + colors[2] / 3;
    Rgb([br, br, br])
}