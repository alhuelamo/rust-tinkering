pub fn binary_search<T>(items: &Vec<T>, target: T) -> Option<T>
    where T: PartialOrd
{
    let mut low = 0;
    let mut high = items.len();

    while low <= high {
        let mid = (low + high) / 2;
        let guess = items.get(mid).unwrap();
        
        if *guess == target {
            return Some(target);
        }

        if *guess < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bindary_search() {
        let v = vec![-21, -1, 0, 1, 5, 9, 12, 45];
        assert_eq!(binary_search(&v, 0), Some(0));
        assert_eq!(binary_search(&v, 1), Some(1));
        assert_eq!(binary_search(&v, -21), Some(-21));
        assert_eq!(binary_search(&v, 4), None);
    }

}
