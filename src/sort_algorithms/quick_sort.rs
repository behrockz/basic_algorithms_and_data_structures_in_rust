use crate::sort_algorithms::sort::Sort;

pub struct QuickSort;

impl<T> Sort<T> for QuickSort  where T : PartialOrd {
    fn sort(vector: &mut Vec<T>) {
        Self::quick_sort(vector)
    }
}
impl QuickSort {
    fn quick_sort<T>(vector: &mut [T]) where T : PartialOrd {
        if vector.len() <= 1 {
            return;
        }

        let index = Self::partition(vector);
        Self::quick_sort(&mut vector[0..index]);
        Self::quick_sort(&mut vector[index + 1..]);
    }

    fn partition<T>(vector: &mut [T]) -> usize where T : PartialOrd{
        let mut i = 0;

        for j in 0..vector.len() - 1 {
            if vector[j] < vector[vector.len()-1] {
                vector.swap(i, j);
                i += 1;
            }
        }
        vector.swap(i, vector.len() - 1);
        i
    }
}

