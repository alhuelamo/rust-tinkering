pub fn rotate90(mtx: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = mtx.len();
    let mut ret: Vec<Vec<i32>> = vec![vec![0i32; n]; n];
    for i in 0..n {
        for j in 0..n {
            ret[i][j] = mtx[(n-1) - j][i];
        }
    }
    ret
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rotate90() {
        let input = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
        let expected = vec![vec![7,4,1], vec![8,5,2], vec![9,6,3]];
        assert_eq!(rotate90(&input), expected);
    }
}
