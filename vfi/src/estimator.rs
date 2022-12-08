pub mod combinator;

use image::{ImageBuffer, Rgb};

use crate::operations::scale_up;

use self::combinator::{run_algo, AlgoSettings, Algorithm};

pub struct Estimator {
    frame_1: Option<ImageBuffer<Rgb<u8>, Vec<u8>>>,
    frame_2: Option<ImageBuffer<Rgb<u8>, Vec<u8>>>,
    frame_1_filename: Option<String>,
    frame_2_filename: Option<String>,
    algorithm: Algorithm,
    pub settings: AlgoSettings,
    pub description: String,
}

impl Estimator {
    pub fn new() -> Estimator {
        Estimator {
            algorithm: Algorithm::BlockMatching,
            settings: AlgoSettings::default(),
            frame_1: None,
            frame_2: None,
            frame_1_filename: None,
            frame_2_filename: None,
            description: "Default description".to_string(),
        }
    }

    pub fn set_frames_directly(
        &mut self,
        frame_1: ImageBuffer<Rgb<u8>, Vec<u8>>,
        frame_2: ImageBuffer<Rgb<u8>, Vec<u8>>,
    ) {
        assert_eq!(frame_1.dimensions(), frame_2.dimensions());
        self.frame_1 = Some(frame_1);
        self.frame_2 = Some(frame_2);
    }

    pub fn set_frames_from_files(
        &mut self,
        frame_1_filename: &str,
        frame_2_filename: &str,
    ) {
        self.frame_1_filename = Some(String::from(frame_1_filename));
        self.frame_2_filename = Some(String::from(frame_2_filename));
        let frame_1 = image::open(frame_1_filename).unwrap().into_rgb8();
        let frame_2 = image::open(frame_2_filename).unwrap().into_rgb8();
        self.set_frames_directly(frame_1, frame_2);
    }

    pub fn get_frames_size(&self) -> (u32, u32) {
        self.frame_1.as_ref().unwrap().dimensions()
    }

    pub fn set_algorithm(&mut self, algo: Algorithm) {
        self.algorithm = algo;
    }

    pub fn estimate_motion(&mut self) -> Vec<Vec<(f32, f32)>> {
        let flow = run_algo(
            &self.algorithm,
            self.frame_1.as_ref().unwrap(),
            self.frame_2.as_ref().unwrap(),
            self.frame_1_filename.clone().unwrap(),
            self.frame_2_filename.clone().unwrap(),
            &mut self.settings,
        );
        let scale_factor = self.get_frames_size().0 / flow[0].len() as u32;
        scale_up(flow, scale_factor)
    }
}
