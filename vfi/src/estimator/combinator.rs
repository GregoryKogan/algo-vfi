mod bma;

use image::{ImageBuffer, Rgb};

use self::bma::{BmaSettings, BMA};

pub enum Algorithm {
    BlockMatching,
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

pub fn run_algo(
    algo: &Algorithm,
    frame_1: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    frame_2: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    settings: &mut AlgoSettings,
) -> Vec<Vec<(i16, i16)>> {
    match algo {
        Algorithm::BlockMatching => {
            let mut bma = BMA::new();
            let mut algo_settings = &mut settings.block_matching;
            algo_settings.verbose = settings.verbose;
            bma.apply_settings(algo_settings);
            bma.calc_flow(frame_1, frame_2)
        }
    }
}
