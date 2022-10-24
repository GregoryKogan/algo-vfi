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
    estimator.settings.block_matching.block_width = 4;
    estimator.settings.block_matching.search_radius = 7;
    estimator.settings.grayscale = true;
    estimator.settings.smoothing.filter_window_width = 3;

    println!("Estimating...");
    estimator.set_frames_from_files("./input/69.png", "./input/70.png");

    let flow = estimator.estimate_motion();

    println!("Visualizing...");
    visualize_flow(
        &flow,
        1,
        VisualizationMethod::HSEScheme,
    ).save("Flow.png").unwrap();
}
