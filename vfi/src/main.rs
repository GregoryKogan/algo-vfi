use estimator::combinator::Algorithm;
use visualizer::{visualize_flow, VisualizationMethod};
use std::time::Instant;
use crate::estimator::Estimator;

mod estimator;
mod operations;
mod plotter;
mod visualizer;

fn main() {
    let mut estimator = Estimator::new();
    estimator.set_algorithm(Algorithm::BidirectionalBlockMatching);
    estimator.settings.verbose = false;
    estimator.settings.block_matching.block_width = 8;
    estimator.settings.block_matching.search_radius = 7;
    estimator.settings.grayscale = true;
    estimator.settings.conv_edges = true;
    estimator.settings.smoothing.filter_window_width = 3;

    println!("Estimating...");
    estimator.set_frames_from_files("./input/69.png", "./input/70.png");

    let now = Instant::now();
    let flow = estimator.estimate_motion();
    let elapsed_time = now.elapsed();

    println!("{:?}", estimator.get_frames_size());
    println!("{}, {}", flow.len(), flow[0].len());

    println!("Took: {:?}ms", elapsed_time.as_millis());

    println!("Visualizing...");
    visualize_flow(
        &flow,
        1,
        VisualizationMethod::HSEScheme,
    ).save("Flow.png").unwrap();
}
