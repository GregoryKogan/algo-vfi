use crate::operations::add_padding;
use crate::operations::pixel_difference;
use image::{ImageBuffer, Rgb};
use num::integer::div_ceil;

pub struct BmaSettings {
    pub verbose: bool,
    pub block_width: u32,
    pub search_radius: u8,
    pub use_movement_map: bool,
    pub movement_map_min_pix_diff: u16,
    pub movement_map_min_change_percentage: u16,
}

impl BmaSettings {
    pub fn default() -> BmaSettings {
        BmaSettings {
            verbose: false,
            block_width: 16,
            search_radius: 7,
            use_movement_map: true,
            movement_map_min_pix_diff: 10,
            movement_map_min_change_percentage: 30,
        }
    }
}

pub struct BMA {
    block_width: u32,
    search_radius: u8,
    use_movement_map: bool,
    movement_map_min_pix_diff: u16,
    movement_map_min_change_percentage: u16,

    verbose: bool,

    padding: u32,
}

impl BMA {
    pub fn new() -> BMA {
        BMA {
            block_width: 16,
            search_radius: 7,
            use_movement_map: true,
            movement_map_min_pix_diff: 10,
            movement_map_min_change_percentage: 30,

            verbose: false,

            padding: 7 + 16,
        }
    }

    pub fn apply_settings(&mut self, settings: &BmaSettings) {
        self.verbose = settings.verbose;
        self.block_width = settings.block_width;
        self.search_radius = settings.search_radius;
        self.padding = self.search_radius as u32 + self.block_width;
        self.use_movement_map = settings.use_movement_map;
        self.movement_map_min_pix_diff = settings.movement_map_min_pix_diff;
        self.movement_map_min_change_percentage = settings.movement_map_min_change_percentage;
    }
}

impl BMA {
    fn has_block_moved(
        &self,
        block_i: u32,
        block_j: u32,
        frame_1: &ImageBuffer<Rgb<u8>, Vec<u8>>,
        frame_2: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    ) -> bool {
        let mut changed_pixels: u32 = 0;
        for x in 0..self.block_width {
            for y in 0..self.block_width {
                let pix_x = block_j * self.block_width + x;
                let pix_y = block_i * self.block_width + y;
                if pixel_difference(frame_1[(pix_x, pix_y)], frame_2[(pix_x, pix_y)])
                    > self.movement_map_min_pix_diff
                {
                    changed_pixels += 1;
                }
            }
        }
        if changed_pixels == 0 {
            return false;
        }
        let change_percentage = self.block_width.pow(2) * 100 / changed_pixels;
        change_percentage > self.movement_map_min_change_percentage as u32
    }

    fn get_motion_vector(
        &self,
        img_1: &ImageBuffer<Rgb<u8>, Vec<u8>>,
        img_2: &ImageBuffer<Rgb<u8>, Vec<u8>>,
        block_i: u32,
        block_j: u32,
    ) -> (i16, i16, u32) {
        let mut error = u32::MAX;
        let mut motion_vector: (i16, i16, u32) = (0, 0, error);

        for x_offset in self.search_radius as i16 * (-1)..self.search_radius as i16 {
            for y_offset in self.search_radius as i16 * (-1)..self.search_radius as i16 {
                let mut cur_error: u32 = 0;
                for x in 0..self.block_width {
                    for y in 0..self.block_width {
                        let pix_x = self.padding + block_j * self.block_width + x;
                        let pix_y = self.padding + block_i * self.block_width + y;
                        cur_error += (pixel_difference(
                            img_1[(pix_x, pix_y)],
                            img_2[(
                                (pix_x as i64 + x_offset as i64) as u32,
                                (pix_y as i64 + y_offset as i64) as u32,
                            )],
                        ) as u32)
                            .pow(2);
                    }
                }
                cur_error /= self.block_width * self.block_width;
                if cur_error < error {
                    error = cur_error;
                    motion_vector = (x_offset, y_offset, cur_error);
                }
            }
        }

        motion_vector
    }

    pub fn calc_flow(
        &self,
        frame_1: &ImageBuffer<Rgb<u8>, Vec<u8>>,
        frame_2: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    ) -> Vec<Vec<(i16, i16, u32)>> {
        let pf1 = add_padding(frame_1, self.padding);
        let pf2 = add_padding(frame_2, self.padding);

        let (width, height) = frame_1.dimensions();
        let hor_blocks = div_ceil(width, self.block_width);
        let ver_blocks = div_ceil(height, self.block_width);

        let mut flow = vec![vec![(0i16, 0i16, u32::MAX); hor_blocks as usize]; ver_blocks as usize];

        for block_i in 0..ver_blocks {
            for block_j in 0..hor_blocks {
                if !self.use_movement_map
                    || (self.use_movement_map
                        && self.has_block_moved(block_i, block_j, frame_1, frame_2))
                {
                    let motion_vector = self.get_motion_vector(&pf1, &pf2, block_i, block_j);
                    flow[block_i as usize][block_j as usize] = motion_vector;

                    if self.verbose && (motion_vector.0, motion_vector.1) != (0, 0) {
                        println!(
                            "Block {},{} -> ({}, {}), error: {}",
                            block_j, block_i, motion_vector.0, motion_vector.1, motion_vector.2
                        );
                    }
                }
            }
        }

        flow
    }
}
