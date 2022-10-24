use std::f64::consts::PI;

use image::{ImageBuffer, Rgb, RgbImage};

use crate::plotter::draw_line;
use crate::plotter::draw_rect;

fn visualize_as_vector_field(
    vf: &Vec<Vec<(f32, f32)>>,
    block_size: u32,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (height, width) = (
        vf.len() as u32 * block_size,
        vf[0].len() as u32 * block_size,
    );
    let mut img: RgbImage = ImageBuffer::new(width, height);
    for i in 0..vf.len() {
        for j in 0..vf[i].len() {
            if vf[i][j] == (0.0, 0.0) {
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

fn cartesian_to_polar(dx: f32, dy: f32) -> (f32, f32) {
    let magnitude = (dx.powf(2.0) + dy.powf(2.0)).sqrt();
    let angle_rad = (dy).atan2(dx);
    let angle = (360.0 + 180.0 * angle_rad / PI as f32) % 360.0;
    (magnitude, angle)
}

fn hsv_to_rgb(h_val: f32, s_val: f32, v_val: f32) -> Option<Rgb<u8>> {
    if h_val > 360.0 || h_val < 0.0 || s_val > 100.0 || s_val < 0.0 || v_val > 100.0 || v_val < 0.0
    {
        return None;
    }

    let s = s_val / 100.0;
    let v = v_val / 100.0;
    let c = s * v;
    let x = c * (1.0 - ((h_val / 60.0 % 2.0) - 1.0).abs());
    let m = v - c;
    let r: f32;
    let g: f32;
    let b: f32;

    if h_val >= 0.0 && h_val < 60.0 {
        r = c;
        g = x;
        b = 0.0;
    } else if h_val >= 60.0 && h_val < 120.0 {
        r = x;
        g = c;
        b = 0.0;
    } else if h_val >= 120.0 && h_val < 180.0 {
        r = 0.0;
        g = c;
        b = x;
    } else if h_val >= 180.0 && h_val < 240.0 {
        r = 0.0;
        g = x;
        b = c;
    } else if h_val >= 240.0 && h_val < 300.0 {
        r = x;
        g = 0.0;
        b = c;
    } else {
        r = c;
        g = 0.0;
        b = x;
    }
    let ir = ((r + m) * 255.0) as u8;
    let ig = ((g + m) * 255.0) as u8;
    let ib = ((b + m) * 255.0) as u8;

    Some(Rgb([ir, ig, ib]))
}

fn vector_to_color(dx: f32, dy: f32, norm_factor: f32) -> Rgb<u8> {
    let (magnitude, angle) = cartesian_to_polar(dx, dy);
    let norm_magnitude = (magnitude / norm_factor * 100.0).clamp(0.0, 100.0);
    hsv_to_rgb(angle, 100.0, norm_magnitude).unwrap()
}

fn visualize_as_hsv_scheme(
    vf: &Vec<Vec<(f32, f32)>>,
    block_size: u32,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (height, width) = (
        vf.len() as u32 * block_size,
        vf[0].len() as u32 * block_size,
    );
    let mut img: RgbImage = ImageBuffer::new(width, height);

    let mut mags: Vec<f32> = Vec::new();
    for i in 0..vf.len() {
        for j in 0..vf[i].len() {
           mags.push((vf[i][j].0.powf(2.0) + vf[i][j].1.powf(2.0)).sqrt());
        }
    }
    mags.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let med_mag = mags[mags.len() / 10 * 9];

    for i in 0..vf.len() {
        for j in 0..vf[i].len() {
            let color = vector_to_color(vf[i][j].0, vf[i][j].1, med_mag);
            let x = j as u32 * block_size;
            let y = i as u32 * block_size;
            draw_rect(&mut img, x, y, block_size, block_size, color);
        }
    }

    img
}

#[allow(dead_code)]
pub enum VisualizationMethod {
    VectorField,
    HSEScheme,
}

pub fn visualize_flow(
    vf: &Vec<Vec<(f32, f32)>>,
    block_size: u32,
    method: VisualizationMethod,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    match method {
        VisualizationMethod::VectorField => visualize_as_vector_field(vf, block_size),
        VisualizationMethod::HSEScheme => visualize_as_hsv_scheme(vf, block_size),
    }
}
