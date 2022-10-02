mod bidirectional_flow;
mod bma;
mod smoothing;

use image::{ImageBuffer, Rgb};

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
) -> Vec<Vec<(i16, i16)>> {
    let bma = get_bma(settings);
    remove_flow_error_data(&bma.calc_flow(frame_1, frame_2))
}

fn run_bidirectional_bma(
    frame_1: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    frame_2: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    settings: &mut AlgoSettings,
) -> Vec<Vec<(i16, i16)>> {
    let bma = get_bma(settings);
    let forward_flow = remove_flow_error_data(&bma.calc_flow(frame_1, frame_2));
    let backward_flow = remove_flow_error_data(&bma.calc_flow(frame_2, frame_1));
    combine_bidirectional_flows(&forward_flow, &backward_flow)
}

fn run_smoothed_bma(
    frame_1: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    frame_2: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    settings: &mut AlgoSettings,
) -> Vec<Vec<(i16, i16)>> {
    let bma = get_bma(settings);
    let error_flow = bma.calc_flow(frame_1, frame_2);
    smooth_error_flow(&error_flow, settings.smoothing.filter_window_width)
}

fn run_smoothed_bidirectional_bma(
    frame_1: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    frame_2: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    settings: &mut AlgoSettings,
) -> Vec<Vec<(i16, i16)>> {
    let bma = get_bma(settings);
    let forward_error_flow = bma.calc_flow(frame_1, frame_2);
    let backward_error_flow = bma.calc_flow(frame_2, frame_1);
    let forward_flow =
        smooth_error_flow(&forward_error_flow, settings.smoothing.filter_window_width);
    let backward_flow =
        smooth_error_flow(&backward_error_flow, settings.smoothing.filter_window_width);
    combine_bidirectional_flows(&forward_flow, &backward_flow)
}

pub fn run_algo(
    algo: &Algorithm,
    frame_1: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    frame_2: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    settings: &mut AlgoSettings,
) -> Vec<Vec<(i16, i16)>> {
    match algo {
        Algorithm::BlockMatching => run_bma(frame_1, frame_2, settings),
        Algorithm::BidirectionalBlockMatching => run_bidirectional_bma(frame_1, frame_2, settings),
        Algorithm::SmoothedBlockMatching => run_smoothed_bma(frame_1, frame_2, settings),
        Algorithm::SmoothedBidirectionalBlockMatching => {
            run_smoothed_bidirectional_bma(frame_1, frame_2, settings)
        }
    }
}
