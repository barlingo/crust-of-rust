use super::Sorter;
use std::fmt::Debug as fmtDebug;

pub struct MergeSort;

impl Sorter for MergeSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord + Copy + fmtDebug,
    {
        if slice.len() <= 1 {
            return;
        }
        let mid = slice.len() / 2;
        Self::sort(&mut slice[mid..]);
        Self::sort(&mut slice[..mid]);
        let mut sorted_slice = slice.to_vec();
        merge(&slice[..mid], &slice[mid..], &mut sorted_slice);
        slice.copy_from_slice(&sorted_slice);
    }
}

fn merge<T: Ord + Copy>(l_slice: &[T], r_slice: &[T], sorted: &mut [T]) {
    let (mut l_idx, mut r_idx, mut idx) = (0, 0, 0);
    while l_idx < l_slice.len() && r_idx < r_slice.len() {
        if l_slice[l_idx] < r_slice[r_idx] {
            sorted[idx] = l_slice[l_idx];
            idx += 1;
            l_idx += 1;
        } else {
            sorted[idx] = r_slice[r_idx];
            idx += 1;
            r_idx += 1;
        }
    }
    if l_idx < l_slice.len() {
        sorted[idx..].copy_from_slice(&l_slice[l_idx..]);
    }
    if r_idx < r_slice.len() {
        sorted[idx..].copy_from_slice(&r_slice[r_idx..]);
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
