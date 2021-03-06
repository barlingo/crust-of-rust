mod bubble_sort;
mod insertion_sort;
mod merge_sort;
mod quick_sort;
mod selection_sort;
use std::fmt::Debug as fmtDebug;

pub trait Sorter {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord + Copy + fmtDebug;
}

pub fn sort<T, S>(slice: &mut [T])
where
    T: Ord + Copy + fmtDebug,
    S: Sorter,
{
    S::sort(slice)
}

#[cfg(test)]
mod tests {
    use super::*;
    struct StdSorter;
    impl Sorter for StdSorter {
        fn sort<T>(slice: &mut [T])
        where
            T: Ord,
        {
            slice.sort();
        }
    }
    #[test]
    fn std_works() {
        let mut things = vec![4, 2, 3, 1];
        sort::<_, StdSorter>(&mut things);
        assert_eq!(things, [1, 2, 3, 4]);
    }
}
