use super::Sorter;
use rand::Rng;
use std::fmt::Debug as fmtDebug;

pub struct QuickSort;

impl Sorter for QuickSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord + fmtDebug + Copy,
    {
        match slice.len() {
            0 | 1 => return,
            2 => {
                if slice[1] < slice[0] {
                    slice.swap(0, 1);
                }
                return;
            }
            _ => {}
        }

        let p_idx = pivot(slice);
        let (left, right) = slice.split_at_mut(p_idx);
        println!("{:?} | {:?}", &left, &right);
        Self::sort(left);
        Self::sort(&mut right[1..]); // Exclude the pivot
    }
}
fn pivot<T: Ord>(slice: &mut [T]) -> usize {
    let mut rng = rand::thread_rng();
    let mut p = rng.gen_range(0..slice.len() - 1);
    slice.swap(0, p);
    p = 0;
    for i in 1..slice.len() {
        if slice[i] < slice[p] {
            slice.swap(p + 1, i);
            slice.swap(p, p + 1);
            p += 1;
        }
    }
    p
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn quick_sort_sorted() {
        let mut things = vec![1, 2, 3, 4, 5, 6, 7];
        QuickSort::sort(&mut things);
        assert_eq!(things, [1, 2, 3, 4, 5, 6, 7]);
    }
    #[test]
    fn quick_sort_odd() {
        let mut things = vec![10, 20, 50, 1, 2, 4, 2, 5, 3, 1, 7, 8];
        QuickSort::sort(&mut things);
        assert_eq!(things, [1, 1, 2, 2, 3, 4, 5, 7, 8, 10, 20, 50]);
    }
    #[test]
    fn quick_sort_even() {
        let mut things = vec![4, 2, 5, 3, 1, 6];
        QuickSort::sort(&mut things);
        assert_eq!(things, [1, 2, 3, 4, 5, 6]);
    }
    #[test]
    fn test_pivot() {
        let mut v = vec![4, 6, 1, 8, 11, 3];
        let p = pivot(&mut v);
        for x in 0..v.len() {
            assert!((v[x] < v[p]) == (x < p));
        }
    }
}
