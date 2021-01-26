mod mtx_rotate;

fn quicksort<T>(mut items: Vec<T>) -> Vec<T>
    where T: PartialOrd
{
    if items.len() < 2 {
        return items;
    }
    
    let pivot_index = items.len() / 2;
    let pivot = items.remove(pivot_index);

    let mut lesser: Vec<T> = Vec::new();
    let mut greater: Vec<T> = Vec::new();

    for item in items.into_iter() {
        let target = if item > pivot {
            &mut greater
        } else {
            &mut lesser
        };
        target.push(item);
    }

    let mut ret = Vec::new();
    ret.extend(quicksort(lesser));
    ret.push(pivot);
    ret.extend(quicksort(greater));
    ret
}

fn binary_search<T>(items: &Vec<T>, target: T) -> Option<T>
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
    fn test_quicksort() {
        let v = vec![9, 45, 12, 1, 0, -21, 5, -1];
        let expected = vec![-21, -1, 0, 1, 5, 9, 12, 45];
        let actual = quicksort(v);
        assert_eq!(actual, expected);
    }
    
    #[test]
    fn test_bindary_search() {
        let v = vec![-21, -1, 0, 1, 5, 9, 12, 45];
        assert_eq!(binary_search(&v, 0), Some(0));
        assert_eq!(binary_search(&v, 1), Some(1));
        assert_eq!(binary_search(&v, -21), Some(-21));
        assert_eq!(binary_search(&v, 4), None);
    }

}
