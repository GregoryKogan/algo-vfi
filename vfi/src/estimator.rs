pub mod combinator;

use image::{ImageBuffer, Rgb};

use self::combinator::{run_algo, AlgoSettings, Algorithm};

pub struct Estimator {
    frame_1: Option<ImageBuffer<Rgb<u8>, Vec<u8>>>,
    frame_2: Option<ImageBuffer<Rgb<u8>, Vec<u8>>>,
    algorithm: Algorithm,
    pub settings: AlgoSettings,
}

impl Estimator {
    pub fn new() -> Estimator {
        Estimator {
            algorithm: Algorithm::BlockMatching,
            settings: AlgoSettings::default(),
            frame_1: None,
            frame_2: None,
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

    pub fn set_algorithm(&mut self, algo: Algorithm) {
        self.algorithm = algo;
    }

    pub fn estimate_motion(&mut self) -> Vec<Vec<(i16, i16)>> {
        run_algo(
            &self.algorithm,
            self.frame_1.as_ref().unwrap(),
            self.frame_2.as_ref().unwrap(),
            &mut self.settings,
        )
    }
}
