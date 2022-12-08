use std::io::Write;
use std::process::Command;
use std::{time::Instant, fs::File};
use std::fs;
use crate::{estimator::{Estimator, combinator::Algorithm}, compensator::compensate, visualizer::{visualize_flow, VisualizationMethod}};


fn get_every_estimator_setting() -> Vec<Estimator> {
    let mut estimator_1 = Estimator::new();
    estimator_1.set_algorithm(Algorithm::BlockMatching);
    estimator_1.settings.block_matching.block_width = 8;
    estimator_1.settings.block_matching.search_radius = 7;
    estimator_1.description = "BMA(8-7)".to_string();

    let mut estimator_2 = Estimator::new();
    estimator_2.set_algorithm(Algorithm::BidirectionalBlockMatching);
    estimator_2.settings.block_matching.block_width = 8;
    estimator_2.settings.block_matching.search_radius = 7;
    estimator_2.description = "BDBMA(8-7)".to_string();

    let mut estimator_3 = Estimator::new();
    estimator_3.set_algorithm(Algorithm::SmoothedBlockMatching);
    estimator_3.settings.block_matching.block_width = 8;
    estimator_3.settings.block_matching.search_radius = 7;
    estimator_3.settings.smoothing.filter_window_width = 3;
    estimator_3.description = "SBMA(8-7-3)".to_string();

    let mut estimator_4 = Estimator::new();
    estimator_4.set_algorithm(Algorithm::SmoothedBidirectionalBlockMatching);
    estimator_4.settings.block_matching.block_width = 8;
    estimator_4.settings.block_matching.search_radius = 7;
    estimator_4.settings.smoothing.filter_window_width = 3;
    estimator_4.description = "SBDBMA(8-7-3)".to_string();

    let mut estimator_5 = Estimator::new();
    estimator_5.set_algorithm(Algorithm::BlockMatching);
    estimator_5.settings.block_matching.block_width = 16;
    estimator_5.settings.block_matching.search_radius = 7;
    estimator_5.description = "BMA(16-7)".to_string();

    let mut estimator_6 = Estimator::new();
    estimator_6.set_algorithm(Algorithm::BidirectionalBlockMatching);
    estimator_6.settings.block_matching.block_width = 16;
    estimator_6.settings.block_matching.search_radius = 7;
    estimator_6.description = "BDBMA(16-7)".to_string();

    let mut estimator_7 = Estimator::new();
    estimator_7.set_algorithm(Algorithm::SmoothedBlockMatching);
    estimator_7.settings.block_matching.block_width = 16;
    estimator_7.settings.block_matching.search_radius = 7;
    estimator_7.settings.smoothing.filter_window_width = 3;
    estimator_7.description = "SBMA(16-7-3)".to_string();

    let mut estimator_8 = Estimator::new();
    estimator_8.set_algorithm(Algorithm::SmoothedBidirectionalBlockMatching);
    estimator_8.settings.block_matching.block_width = 16;
    estimator_8.settings.block_matching.search_radius = 7;
    estimator_8.settings.smoothing.filter_window_width = 3;
    estimator_8.description = "SBDBMA(16-7-3)".to_string();

    let mut estimator_9 = Estimator::new();
    estimator_9.set_algorithm(Algorithm::BlockMatching);
    estimator_9.settings.block_matching.block_width = 8;
    estimator_9.settings.block_matching.search_radius = 7;
    estimator_9.settings.grayscale = true;
    estimator_9.description = "GBMA(8-7)".to_string();

    let mut estimator_10 = Estimator::new();
    estimator_10.set_algorithm(Algorithm::BidirectionalBlockMatching);
    estimator_10.settings.block_matching.block_width = 8;
    estimator_10.settings.block_matching.search_radius = 7;
    estimator_10.settings.grayscale = true;
    estimator_10.description = "GBDBMA(8-7)".to_string();

    let mut estimator_11 = Estimator::new();
    estimator_11.set_algorithm(Algorithm::SmoothedBlockMatching);
    estimator_11.settings.block_matching.block_width = 8;
    estimator_11.settings.block_matching.search_radius = 7;
    estimator_11.settings.smoothing.filter_window_width = 3;
    estimator_11.settings.grayscale = true;
    estimator_11.description = "GSBMA(8-7-3)".to_string();

    let mut estimator_12 = Estimator::new();
    estimator_12.set_algorithm(Algorithm::SmoothedBidirectionalBlockMatching);
    estimator_12.settings.block_matching.block_width = 8;
    estimator_12.settings.block_matching.search_radius = 7;
    estimator_12.settings.smoothing.filter_window_width = 3;
    estimator_12.settings.grayscale = true;
    estimator_12.description = "GSBDBMA(8-7-3)".to_string();

    let mut estimator_13 = Estimator::new();
    estimator_13.set_algorithm(Algorithm::BlockMatching);
    estimator_13.settings.block_matching.block_width = 8;
    estimator_13.settings.block_matching.search_radius = 7;
    estimator_13.settings.grayscale = true;
    estimator_13.settings.conv_edges = true;
    estimator_13.description = "EBMA(8-7)".to_string();

    let mut estimator_14 = Estimator::new();
    estimator_14.set_algorithm(Algorithm::BidirectionalBlockMatching);
    estimator_14.settings.block_matching.block_width = 8;
    estimator_14.settings.block_matching.search_radius = 7;
    estimator_14.settings.grayscale = true;
    estimator_14.settings.conv_edges = true;
    estimator_14.description = "EBDBMA(8-7)".to_string();

    let mut estimator_15 = Estimator::new();
    estimator_15.set_algorithm(Algorithm::SmoothedBlockMatching);
    estimator_15.settings.block_matching.block_width = 8;
    estimator_15.settings.block_matching.search_radius = 7;
    estimator_15.settings.smoothing.filter_window_width = 3;
    estimator_15.settings.grayscale = true;
    estimator_15.settings.conv_edges = true;
    estimator_15.description = "ESBMA(8-7-3)".to_string();

    let mut estimator_16 = Estimator::new();
    estimator_16.set_algorithm(Algorithm::SmoothedBidirectionalBlockMatching);
    estimator_16.settings.block_matching.block_width = 8;
    estimator_16.settings.block_matching.search_radius = 7;
    estimator_16.settings.smoothing.filter_window_width = 3;
    estimator_16.settings.grayscale = true;
    estimator_16.settings.conv_edges = true;
    estimator_16.description = "ESBDBMA(8-7-3)".to_string();

    let mut estimator_17 = Estimator::new();
    estimator_17.set_algorithm(Algorithm::SmoothedBlockMatching);
    estimator_17.settings.block_matching.block_width = 8;
    estimator_17.settings.block_matching.search_radius = 7;
    estimator_17.settings.smoothing.filter_window_width = 5;
    estimator_17.description = "SBMA(8-7-5)".to_string();

    let mut estimator_18 = Estimator::new();
    estimator_18.set_algorithm(Algorithm::SmoothedBidirectionalBlockMatching);
    estimator_18.settings.block_matching.block_width = 8;
    estimator_18.settings.block_matching.search_radius = 7;
    estimator_18.settings.smoothing.filter_window_width = 5;
    estimator_18.description = "SBDBMA(8-7-5)".to_string();

    let mut estimator_19 = Estimator::new();
    estimator_19.set_algorithm(Algorithm::LucasKanade);
    estimator_19.description = "Lucas-Kanade".to_string();

    let mut estimator_20 = Estimator::new();
    estimator_20.set_algorithm(Algorithm::GunnarFarneback);
    estimator_20.description = "Gunnar-Farneback".to_string();

    let estimators = vec![
        estimator_1,
        estimator_2,
        estimator_3,
        estimator_4,
        estimator_5,
        estimator_6,
        estimator_7,
        estimator_8,
        estimator_9,
        estimator_10,
        estimator_11,
        estimator_12,
        estimator_13,
        estimator_14,
        estimator_15,
        estimator_16,
        estimator_17,
        estimator_18,
        estimator_19,
        estimator_20,
    ] as Vec<Estimator>;

    estimators
}


fn create_video(algo_name: String) {
    Command::new("ffmpeg").args([
        "-framerate", 
        "30",
        "-pattern_type", 
        "glob",
        "-i",
        &format!("./Results/{}/frames/*.png", algo_name),
        "-c:v", 
        "libx264",
        "-pix_fmt",
        "yuv420p",
        &format!("./Results/{}/{}-Interpolated30fps.mp4", algo_name, algo_name),
    ]).spawn().expect("failed to execute process");

    Command::new("ffmpeg").args([
        "-framerate", 
        "30",
        "-pattern_type", 
        "glob",
        "-i",
        &format!("./Results/{}/flow/*.png", algo_name),
        "-c:v", 
        "libx264",
        "-pix_fmt",
        "yuv420p",
        &format!("./Results/{}/{}-Flow30fps.mp4", algo_name, algo_name),
    ]).spawn().expect("failed to execute process");
}


pub fn test() {
    fs::remove_dir_all("./Results").ok();
    fs::create_dir_all("./Results").unwrap();

    let estimators = get_every_estimator_setting();
    for mut estimator in estimators {
        println!("Computing: {}", estimator.description);
        fs::create_dir_all(format!("./Results/{}", estimator.description)).unwrap();
        fs::create_dir_all(format!("./Results/{}/frames", estimator.description)).unwrap();
        fs::create_dir_all(format!("./Results/{}/flow", estimator.description)).unwrap();

        let now = Instant::now();

        let input_frames = 5;
        for frame_index in 1..input_frames {
            println!("{}: {}/{} frames done", estimator.description, frame_index, input_frames);
            let frame_1_filename = &format!("./input/{}.png", frame_index);
            let frame_2_filename = &format!("./input/{}.png", frame_index + 1);

            estimator.set_frames_from_files(frame_1_filename, frame_2_filename);
            let flow = estimator.estimate_motion();

            image::open(frame_1_filename).unwrap().into_rgb8().save(
                format!("./Results/{}/frames/{}.png", 
                estimator.description, 
                frame_index * 2 - 1)
            ).unwrap();
            compensate(
                frame_1_filename, 
                frame_2_filename, 
                &flow
            ).save(
                format!("./Results/{}/frames/{}.png", 
                estimator.description, 
                frame_index * 2)
            ).unwrap();
        
            visualize_flow(
                &flow,
                1,
                VisualizationMethod::HSEScheme,
            ).save(
                format!("./Results/{}/flow/{}.png", 
                estimator.description, 
                frame_index)
            ).unwrap();
        }

        let elapsed_time = now.elapsed();
        println!("Took: {:?}s", elapsed_time.as_secs());

        let mut file = File::create(
            format!("./Results/{}/Performance.txt", estimator.description)
        ).unwrap();
        let message = format!(
            "Total time for {} frames: {:?}s\nAverage per frame: {:?}ms", 
            input_frames,
            elapsed_time.as_secs(), 
            elapsed_time.as_millis() / (input_frames - 1)
        );
        file.write_all(message.as_bytes()).unwrap();

        create_video(estimator.description);
    }
}
