use super::Sorter;

pub struct InsertionSort;

impl Sorter for InsertionSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        for unsorted in 1..slice.len() {
            let i = unsorted;
            while i > 0 && slice[i - 1] > i {
                slice.swap(i - 1, i);
                i -= 1;
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn insertion_sort_odd() {
        let mut things = vec![4, 2, 5, 3, 1];
        InsertionSort::sort(&mut things);
        assert_eq!(things, [1, 2, 3, 4, 5]);
    }
    #[test]
    fn insertion_sort_even() {
        let mut things = vec![2, 5, 3, 1];
        InsertionSort::sort(&mut things);
        assert_eq!(things, [1, 2, 3, 5]);
    }
}
