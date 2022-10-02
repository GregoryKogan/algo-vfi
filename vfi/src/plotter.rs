use image::{ImageBuffer, Rgb};

pub fn draw_line(
    img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    p1: (u32, u32),
    p2: (u32, u32),
    color: Rgb<u8>,
    width: u32,
) {
    // Create local variables for moving start point
    let mut x0 = p1.0 as i32;
    let mut y0 = p1.1 as i32;
    let x1 = p2.0 as i32;
    let y1 = p2.1 as i32;

    // Get absolute x/y offset
    let dx: i32 = if x0 > x1 { x0 - x1 } else { x1 - x0 };
    let dy: i32 = if y0 > y1 { y0 - y1 } else { y1 - y0 };

    // Get slopes
    let sx: i32 = if x0 < x1 { 1 } else { -1 };
    let sy: i32 = if y0 < y1 { 1 } else { -1 };

    // Initialize error
    let mut err: i32 = if dx > dy { dx as i32 } else { -(dy as i32) } / 2;
    let mut err2;

    let mut first = true;
    loop {
        // Set pixel
        for ox in 0..width {
            for oy in 0..width {
                if first {
                    img.put_pixel(x0 as u32 + ox, y0 as u32 + oy, Rgb([0, 255, 0]));
                    first = false;
                } else {
                    img.put_pixel(x0 as u32 + ox, y0 as u32 + oy, color);
                }
            }
        }

        // Check end condition
        if x0 == x1 && y0 == y1 {
            break;
        };

        // Store old error
        err2 = 2 * err;

        // Adjust error and start position
        if err2 > -dx {
            err -= dy;
            x0 += sx;
        }
        if err2 < dy {
            err += dx;
            y0 += sy;
        }
    }
}

pub fn draw_rect(
    img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    color: Rgb<u8>,
) {
    for ox in 0..w {
        for oy in 0..h {
            img.put_pixel(x + ox, y + oy, color);
        }
    }
}
