mod bma;

use image::{ImageBuffer, Rgb};

use self::bma::{BmaSettings, BMA};

pub enum Algorithm {
    BlockMatching,
    BidirectionalBlockMatching,
}

pub struct AlgoSettings {
    pub verbose: bool,
    pub block_matching: BmaSettings,
}

impl AlgoSettings {
    pub fn default() -> AlgoSettings {
        AlgoSettings {
            verbose: false,
            block_matching: BmaSettings::default(),
        }
    }
}

fn run_bma(frame_1: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    frame_2: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    settings: &mut AlgoSettings) -> Vec<Vec<(i16, i16)>> {
        let mut bma = BMA::new();
        let mut algo_settings = &mut settings.block_matching;
        algo_settings.verbose = settings.verbose;
        bma.apply_settings(algo_settings);
        bma.calc_flow(frame_1, frame_2)
}

fn run_bidirectional_bma(frame_1: &ImageBuffer<Rgb<u8>, Vec<u8>>, frame_2: &ImageBuffer<Rgb<u8>, Vec<u8>>, settings: &mut AlgoSettings) -> Vec<Vec<(i16, i16)>> {
    let mut bma = BMA::new();
    let mut algo_settings = &mut settings.block_matching;
    algo_settings.verbose = settings.verbose;
    bma.apply_settings(algo_settings);

    let forward_flow = bma.calc_flow(frame_1, frame_2);
    let backward_flow = bma.calc_flow(frame_2, frame_1);
    let mut flow = vec![vec![(0i16, 0i16); forward_flow[0].len()]; forward_flow.len()];
    for i in 0..forward_flow.len() {
        for j in 0..forward_flow[0].len() {
            let fx = (forward_flow[i][j].0 - backward_flow[i][j].0) / 2;
            let fy = (forward_flow[i][j].1 - backward_flow[i][j].1) / 2;
            flow[i][j] = (fx, fy);
        }
    }

    flow
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
    }
}
