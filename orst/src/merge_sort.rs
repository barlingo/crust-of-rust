use super::Sorter;
use std::fmt::Debug as fmtDebug;

pub struct MergeSort;

impl Sorter for MergeSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord + std::fmt::Debug + Copy,
    {
        if slice.len() <= 1 {
            return;
        }

        // println!("{:?}", &slice);
        let mid = slice.len() / 2;
        let (left, right) = slice.split_at_mut(mid);
        Self::sort(right);
        Self::sort(left);
        merge(left, right);
        println!("{:?} | {:?}", &left, &right);
    }
}

fn merge<T: Ord + fmtDebug + Copy>(left: &mut [T], right: &mut [T]) {
    let size = left.len() + right.len();
    let mut result: Vec<T> = Vec::with_capacity(size);
    let (mut l, mut r) = (0, 0);
    while l < left.len() && r < right.len() {
        if left[l] < right[r] {
            result.push(left[l]);
            l += 1;
        } else {
            result.push(right[r]);
            r += 1;
        }
    }
    while l < left.len() {
        result.push(left[l]);
        l += 1;
    }
    while r < right.len() {
        result.push(right[r]);
        r += 1;
    }

    for (idx, val) in result.iter().enumerate() {
        if idx < left.len() {
            left[idx] = *val;
        } else {
            right[idx - left.len()] = *val;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn merge_sort_odd() {
        let mut things = vec![10, 20, 50, 1, 2, 4, 2, 5, 3, 1, 7, 8];
        MergeSort::sort(&mut things);
        assert_eq!(things, [1, 1, 2, 2, 3, 4, 5, 7, 8, 10, 20, 50]);
    }
    #[test]
    fn merge_sort_even() {
        let mut things = vec![4, 2, 5, 3, 1, 6];
        MergeSort::sort(&mut things);
        assert_eq!(things, [1, 2, 3, 4, 5, 6]);
    }
}
