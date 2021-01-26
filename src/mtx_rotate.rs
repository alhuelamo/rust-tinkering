fn rotate90(mtx: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = mtx.len();
    let mut ret: Vec<Vec<i32>> = vec![vec![0i32; n]; n];
    for i in 0..n {
        for j in 0..n {
            ret[i][j] = mtx[(n-1) - j][i];
        }
    }
    ret
}
