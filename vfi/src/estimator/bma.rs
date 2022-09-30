use crate::operations::add_padding;
use crate::operations::pixel_difference;
use image::{ImageBuffer, Rgb};
use num::integer::div_ceil;

pub struct BMA {
    frame_1: Option<ImageBuffer<Rgb<u8>, Vec<u8>>>,
    frame_2: Option<ImageBuffer<Rgb<u8>, Vec<u8>>>,
    width: u32,
    height: u32,

    verbose: bool,

    block_width: u32,
    search_radius: u8,
    use_movement_map: bool,
    movement_map_min_pix_diff: u16,
    movement_map_min_change_percentage: u16,

    padding: u32,
}

impl BMA {
    pub fn new() -> BMA {
        BMA {
            frame_1: None,
            frame_2: None,
            width: 0,
            height: 0,
            verbose: false,

            block_width: 16,
            search_radius: 7,
            use_movement_map: true,
            movement_map_min_pix_diff: 10,
            movement_map_min_change_percentage: 30,

            padding: 7 + 16,
        }
    }

    pub fn set_frames(
        &mut self,
        frame_1: ImageBuffer<Rgb<u8>, Vec<u8>>,
        frame_2: ImageBuffer<Rgb<u8>, Vec<u8>>,
    ) {
        assert_eq!(frame_1.dimensions(), frame_2.dimensions());
        (self.width, self.height) = frame_1.dimensions();
        self.frame_1 = Some(frame_1);
        self.frame_2 = Some(frame_2);
    }

    pub fn set_verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    pub fn set_block_width(&mut self, b_width: u32) {
        self.block_width = b_width;
        self.padding = self.search_radius as u32 + self.block_width;
    }

    pub fn set_search_radius(&mut self, s_radius: u8) {
        self.search_radius = s_radius;
        self.padding = self.search_radius as u32 + self.block_width;
    }

    pub fn set_use_movement_map(&mut self, use_mov_map: bool) {
        self.use_movement_map = use_mov_map;
    }

    pub fn set_movement_map_min_pix_diff(&mut self, min_pix_diff: u16) {
        self.movement_map_min_pix_diff = min_pix_diff;
    }

    pub fn set_movement_map_min_change_percentage(&mut self, min_change_percentage: u16) {
        self.movement_map_min_change_percentage = min_change_percentage;
    }
}

impl BMA {
    fn has_block_moved(&self, block_i: u32, block_j: u32) -> bool {
        let mut changed_pixels: u32 = 0;
        for x in 0..self.block_width {
            for y in 0..self.block_width {
                let pix_x = block_j * self.block_width + x;
                let pix_y = block_i * self.block_width + y;
                if pixel_difference(
                    self.frame_1.as_ref().unwrap()[(pix_x, pix_y)],
                    self.frame_2.as_ref().unwrap()[(pix_x, pix_y)],
                ) > self.movement_map_min_pix_diff
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
    ) -> (i16, i16) {
        let mut motion_vector: (i16, i16) = (0, 0);
        let mut error = u32::MAX;

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
                    motion_vector = (x_offset, y_offset);
                }
            }
        }

        motion_vector
    }

    pub fn calc_flow(&self) -> Vec<Vec<(i16, i16)>> {
        let pf1 = add_padding(&self.frame_1.as_ref().unwrap(), self.padding);
        let pf2 = add_padding(&self.frame_2.as_ref().unwrap(), self.padding);

        let hor_blocks = div_ceil(self.width, self.block_width);
        let ver_blocks = div_ceil(self.height, self.block_width);

        let mut flow = vec![vec![(0i16, 0i16); hor_blocks as usize]; ver_blocks as usize];

        for block_i in 0..ver_blocks {
            for block_j in 0..hor_blocks {
                if !self.use_movement_map
                    || (self.use_movement_map && self.has_block_moved(block_i, block_j))
                {
                    let motion_vector = self.get_motion_vector(&pf1, &pf2, block_i, block_j);
                    flow[block_i as usize][block_j as usize] = motion_vector;

                    if self.verbose && motion_vector != (0, 0) {
                        println!("Block {},{} -> {:?}", block_j, block_i, motion_vector);
                    }
                }
            }
        }

        flow
    }
}
