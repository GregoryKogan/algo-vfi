pub fn combine_bidirectional_flows(
    forward_flow: &Vec<Vec<(i16, i16)>>,
    backward_flow: &Vec<Vec<(i16, i16)>>,
) -> Vec<Vec<(i16, i16)>> {
    assert_eq!(forward_flow.len(), backward_flow.len());
    assert_eq!(forward_flow[0].len(), backward_flow[0].len());

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
