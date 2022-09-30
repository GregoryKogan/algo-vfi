use estimator::Algorithm;

use crate::estimator::Estimator;

mod estimator;
mod operations;

fn main() {
    let mut estimator = Estimator::new();
    estimator.set_verbose(true);
    estimator.set_algorithm(Algorithm::BlockMatching);
    estimator.bma.set_block_width(8);
    estimator.bma.set_search_radius(7);
    estimator.bma.set_use_movement_map(true);
    estimator.bma.set_movement_map_min_pix_diff(10);
    estimator.bma.set_movement_map_min_change_percentage(30);

    let frame_1 = image::open("./input/69.png").unwrap().into_rgb8();
    let frame_2 = image::open("./input/70.png").unwrap().into_rgb8();
    estimator.set_frames(frame_1, frame_2);

    estimator.estimate_motion();
}
