pub struct SmoothingSettings {
    pub filter_window_width: u32,
}

impl SmoothingSettings {
    pub fn default() -> SmoothingSettings {
        SmoothingSettings {
            filter_window_width: 3,
        }
    }
}

fn get_window_weights(
    error_flow: &Vec<Vec<(i16, i16, u32)>>,
    filter_w: u32,
    i: u32,
    j: u32,
) -> Vec<Vec<f64>> {
    let mut weights = vec![vec![0f64; filter_w as usize]; filter_w as usize];
    let center_error = error_flow[i as usize][j as usize].2;
    for y in 0..filter_w {
        for x in 0..filter_w {
            weights[y as usize][x as usize] = center_error as f64
                / error_flow[(i + y - filter_w / 2) as usize][(j + x - filter_w / 2) as usize].2
                    as f64;
        }
    }
    weights
}

fn vector_diff(v1: (i16, i16), v2: (i16, i16)) -> f64 {
    ((v1.0.abs_diff(v2.0).pow(2) + v1.1.abs_diff(v2.1).pow(2)) as f64).sqrt()
}

fn get_median_weighted_vector(
    error_flow: &Vec<Vec<(i16, i16, u32)>>,
    filter_w: u32,
    weights: &Vec<Vec<f64>>,
    i: u32,
    j: u32,
) -> (i16, i16) {
    assert_eq!(weights.len() as u32, filter_w);
    assert_eq!(weights[0].len() as u32, filter_w);

    let mut min_deviation = f64::INFINITY;
    let mut mwv = (0i16, 0i16);
    for vi in 0..filter_w {
        for vj in 0..filter_w {
            let cur_mwv = (
                error_flow[(i + vi - filter_w / 2) as usize][(j + vj - filter_w / 2) as usize].0,
                error_flow[(i + vi - filter_w / 2) as usize][(j + vj - filter_w / 2) as usize].1,
            );
            let mut cur_deviation = 0f64;
            for ni in 0..filter_w {
                for nj in 0..filter_w {
                    let neighbor_v = (
                        error_flow[(i + ni - filter_w / 2) as usize]
                            [(j + nj - filter_w / 2) as usize]
                            .0,
                        error_flow[(i + ni - filter_w / 2) as usize]
                            [(j + nj - filter_w / 2) as usize]
                            .1,
                    );
                    cur_deviation +=
                        weights[ni as usize][nj as usize] * vector_diff(cur_mwv, neighbor_v);
                }
            }
            if cur_deviation < min_deviation {
                min_deviation = cur_deviation;
                mwv = cur_mwv;
            }
        }
    }

    mwv
}

pub fn smooth_error_flow(
    error_flow: &Vec<Vec<(i16, i16, u32)>>,
    filter_w: u32,
) -> Vec<Vec<(i16, i16)>> {
    assert!(filter_w % 2 != 0);
    let half_fw = filter_w / 2;
    let mut flow = vec![vec![(0i16, 0i16); error_flow[0].len()]; error_flow.len()];
    for i in half_fw..error_flow.len() as u32 - half_fw {
        for j in half_fw..error_flow[0].len() as u32 - half_fw {
            let weights = get_window_weights(error_flow, filter_w, i, j);
            flow[i as usize][j as usize] =
                get_median_weighted_vector(error_flow, filter_w, &weights, i, j);
        }
    }

    for i in 0..error_flow.len() {
        for offset in 0..(filter_w / 2) as usize {
            flow[i][offset] = (error_flow[i][offset].0, error_flow[i][offset].1);
            flow[i][error_flow[i].len() - 1 - offset] = (
                error_flow[i][error_flow[i].len() - 1 - offset].0,
                error_flow[i][error_flow[i].len() - 1 - offset].1,
            );
        }
    }
    for j in 0..error_flow[0].len() {
        for offset in 0..(filter_w / 2) as usize {
            flow[offset][j] = (error_flow[offset][j].0, error_flow[offset][j].1);
            flow[error_flow.len() - 1 - offset][j] = (
                error_flow[error_flow.len() - 1 - offset][j].0,
                error_flow[error_flow.len() - 1 - offset][j].1,
            );
        }
    }
    flow
}
