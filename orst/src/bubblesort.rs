use super::Sorter;

pub struct Bubblesort;

impl Sorter for Bubblesort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        let mut swapped = true;
        while swapped {
            swapped = false;
            for i in (0..slice.len() - 1) {
                if slice[i] > slice[i + 1] {
                    slice.swap(i, i + 1);
                    swapped = true;
                }
            }
        }
        //TODO
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bubble_sort_odd() {
        let mut things = vec![4, 2, 5, 3, 1];
        Bubblesort::sort(&mut things);
        assert_eq!(things, [1, 2, 3, 4, 5]);
    }
    #[test]
    fn bubble_sort_even() {
        let mut things = vec![2, 5, 3, 1];
        Bubblesort::sort(&mut things);
        assert_eq!(things, [1, 2, 3, 5]);
    }
}
