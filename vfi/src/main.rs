use estimator::combinator::Algorithm;
use visualizer::{visualize_flow, VisualizationMethod};

use crate::estimator::Estimator;

mod estimator;
mod operations;
mod plotter;
mod visualizer;

fn main() {
    let mut estimator = Estimator::new();
    estimator.set_algorithm(Algorithm::SmoothedBidirectionalBlockMatching);
    estimator.settings.verbose = true;
    estimator.settings.block_matching.block_width = 8;
    estimator.settings.block_matching.search_radius = 7;
    estimator.settings.smoothing.filter_window_width = 3;

    let frame_1 = image::open("./input/69.png").unwrap().into_rgb8();
    let frame_2 = image::open("./input/70.png").unwrap().into_rgb8();
    estimator.set_frames(frame_1, frame_2);

    let flow = estimator.estimate_motion();
    visualize_flow(
        &flow,
        estimator.settings.block_matching.block_width,
        VisualizationMethod::HSEScheme,
    )
    .save("SmoothFlow.png")
    .unwrap();
}
