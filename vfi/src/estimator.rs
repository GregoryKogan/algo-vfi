mod bma;

use self::bma::BMA;
use image::{ImageBuffer, Rgb};

pub enum Algorithm {
    BlockMatching,
}

pub struct Estimator {
    frame_1: Option<ImageBuffer<Rgb<u8>, Vec<u8>>>,
    frame_2: Option<ImageBuffer<Rgb<u8>, Vec<u8>>>,

    verbose: bool,

    algorithm: Algorithm,

    pub bma: BMA,
}

impl Estimator {
    pub fn new() -> Estimator {
        let bma = BMA::new();
        Estimator {
            verbose: false,
            algorithm: Algorithm::BlockMatching,
            frame_1: None,
            frame_2: None,
            bma,
        }
    }

    pub fn set_frames(
        &mut self,
        frame_1: ImageBuffer<Rgb<u8>, Vec<u8>>,
        frame_2: ImageBuffer<Rgb<u8>, Vec<u8>>,
    ) {
        assert_eq!(frame_1.dimensions(), frame_2.dimensions());
        self.frame_1 = Some(frame_1);
        self.frame_2 = Some(frame_2);
    }

    pub fn set_verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
        self.bma.set_verbose(verbose);
    }

    pub fn set_algorithm(&mut self, algo: Algorithm) {
        self.algorithm = algo;
    }

    pub fn estimate_motion(&mut self) -> Vec<Vec<(i16, i16)>> {
        match self.algorithm {
            Algorithm::BlockMatching => {
                self.bma.set_frames(
                    self.frame_1.as_ref().unwrap().clone(),
                    self.frame_2.as_ref().unwrap().clone(),
                );
                self.bma.calc_flow()
            }
        }
    }
}
