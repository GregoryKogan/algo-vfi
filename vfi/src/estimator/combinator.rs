mod bidirectional_flow;
mod bma;
mod smoothing;

use image::{ImageBuffer, Rgb};
use std::process::Command;

use self::{
    bidirectional_flow::combine_bidirectional_flows,
    bma::{BmaSettings, BMA},
    smoothing::{smooth_error_flow, SmoothingSettings},
};

#[allow(dead_code)]
pub enum Algorithm {
    BlockMatching,
    BidirectionalBlockMatching,
    SmoothedBlockMatching,
    SmoothedBidirectionalBlockMatching,
    LucasKanade,
    GunnarFarneback,
}

pub struct AlgoSettings {
    pub verbose: bool,
    pub block_matching: BmaSettings,
    pub smoothing: SmoothingSettings,
}

impl AlgoSettings {
    pub fn default() -> AlgoSettings {
        AlgoSettings {
            verbose: false,
            block_matching: BmaSettings::default(),
            smoothing: SmoothingSettings::default(),
        }
    }
}

fn remove_flow_error_data(error_flow: &Vec<Vec<(i16, i16, u32)>>) -> Vec<Vec<(i16, i16)>> {
    let mut flow = vec![vec![(0i16, 0i16); error_flow[0].len()]; error_flow.len()];
    for i in 0..error_flow.len() {
        for j in 0..error_flow[i].len() {
            flow[i][j] = (error_flow[i][j].0, error_flow[i][j].1);
        }
    }
    flow
}

fn convert_int_flow_to_float(int_flow: &Vec<Vec<(i16, i16)>>) -> Vec<Vec<(f32, f32)>> {
    let mut flow = vec![vec![(0f32, 0f32); int_flow[0].len() as usize]; int_flow.len() as usize];
    for i in 0..int_flow.len() {
        for j in 0..int_flow[0].len() {
            flow[i][j] = (int_flow[i][j].0 as f32, int_flow[i][j].1 as f32);
        }
    }
    flow
}

fn get_bma(settings: &mut AlgoSettings) -> BMA {
    let mut bma = BMA::new();
    let mut algo_settings = &mut settings.block_matching;
    algo_settings.verbose = settings.verbose;
    bma.apply_settings(algo_settings);
    bma
}

fn run_bma(
    frame_1: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    frame_2: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    settings: &mut AlgoSettings,
) -> Vec<Vec<(f32, f32)>> {
    let bma = get_bma(settings);
    let int_flow = remove_flow_error_data(&bma.calc_flow(frame_1, frame_2));
    convert_int_flow_to_float(&int_flow)
}

fn run_bidirectional_bma(
    frame_1: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    frame_2: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    settings: &mut AlgoSettings,
) -> Vec<Vec<(f32, f32)>> {
    let bma = get_bma(settings);
    let forward_flow = remove_flow_error_data(&bma.calc_flow(frame_1, frame_2));
    let backward_flow = remove_flow_error_data(&bma.calc_flow(frame_2, frame_1));
    convert_int_flow_to_float(&combine_bidirectional_flows(&forward_flow, &backward_flow))
}

fn run_smoothed_bma(
    frame_1: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    frame_2: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    settings: &mut AlgoSettings,
) -> Vec<Vec<(f32, f32)>> {
    let bma = get_bma(settings);
    let error_flow = bma.calc_flow(frame_1, frame_2);
    convert_int_flow_to_float(&smooth_error_flow(&error_flow, settings.smoothing.filter_window_width))
}

fn run_smoothed_bidirectional_bma(
    frame_1: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    frame_2: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    settings: &mut AlgoSettings,
) -> Vec<Vec<(f32, f32)>> {
    let bma = get_bma(settings);
    let forward_error_flow = bma.calc_flow(frame_1, frame_2);
    let backward_error_flow = bma.calc_flow(frame_2, frame_1);
    let forward_flow =
        smooth_error_flow(&forward_error_flow, settings.smoothing.filter_window_width);
    let backward_flow =
        smooth_error_flow(&backward_error_flow, settings.smoothing.filter_window_width);
    convert_int_flow_to_float(&combine_bidirectional_flows(&forward_flow, &backward_flow))
}

fn run_executable(exe_name: &str, frame_1_filename: String, frame_2_filename: String)  -> Vec<Vec<(f32, f32)>> {
    let output = Command::new(format!("./src/executables/{}", exe_name))
    .arg(frame_1_filename)
    .arg(frame_2_filename)
    .output()
    .expect("failed to execute process");
    let output = String::from_utf8_lossy(&output.stdout);
    let mut lines = output.lines();

    let mut parts = lines.next().unwrap().split_whitespace().map(|s| s.parse::<u32>());
    let height = parts.next().unwrap().unwrap();
    let width = parts.next().unwrap().unwrap();
    
    let mut flow = vec![vec![(0f32, 0f32); width as usize]; height as usize];
    for i in 0..height as usize {
        for j in 0..width as usize {
            let mut offset = lines.next().unwrap().split_whitespace().map(|s| s.parse::<f32>());
            let offset_x = offset.next().unwrap().unwrap();
            let offset_y = offset.next().unwrap().unwrap();
            flow[i][j] = (offset_x, offset_y);
        }
    }

    flow
}

pub fn run_algo(
    algo: &Algorithm,
    frame_1: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    frame_2: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    frame_1_filename: String,
    frame_2_filename: String,
    settings: &mut AlgoSettings,
) -> Vec<Vec<(f32, f32)>> {
    match algo {
        Algorithm::BlockMatching => run_bma(frame_1, frame_2, settings),
        Algorithm::BidirectionalBlockMatching => run_bidirectional_bma(frame_1, frame_2, settings),
        Algorithm::SmoothedBlockMatching => run_smoothed_bma(frame_1, frame_2, settings),
        Algorithm::SmoothedBidirectionalBlockMatching => {
            run_smoothed_bidirectional_bma(frame_1, frame_2, settings)
        }
        Algorithm::LucasKanade => run_executable("lucas_kanade", frame_1_filename, frame_2_filename),
        Algorithm::GunnarFarneback => run_executable("farneback", frame_1_filename, frame_2_filename)
    }
}
