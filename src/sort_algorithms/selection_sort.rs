use crate::sort_algorithms::sort::Sort;

pub struct SelectionSort;

impl<T> Sort<T> for SelectionSort where T : PartialOrd{
    fn sort(vector: &mut Vec<T>) {
        for i in 0..vector.len() {
            let mut smallest_index = i;
            for j in (i + 1)..vector.len() {
                if vector[j] < vector[smallest_index] {
                    smallest_index = j;
                }
            }
            vector.swap(i, smallest_index);
        }
    }
}
