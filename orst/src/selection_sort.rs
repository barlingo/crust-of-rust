use super::Sorter;

pub struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        // Place the minimum value always at the beginning
        for j in 0..slice.len() {
            let mut min_idx = j;
            for i in j + 1..slice.len() {
                if slice[min_idx] > slice[i] {
                    min_idx = i;
                }
            }
            slice.swap(j, min_idx);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn selectionsort_odd() {
        let mut things = vec![4, 2, 5, 3, 1];
        SelectionSort::sort(&mut things);
        assert_eq!(things, [1, 2, 3, 4, 5]);
    }
    #[test]
    fn selectionsort_even() {
        let mut things = vec![4, 2, 5, 3, 1, 6];
        SelectionSort::sort(&mut things);
        assert_eq!(things, [1, 2, 3, 4, 5, 6]);
    }
}
