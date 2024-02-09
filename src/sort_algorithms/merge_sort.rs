use crate::sort_algorithms::sort::Sort;

pub struct MergeSort;

impl<T> Sort<T> for MergeSort  where T : PartialOrd{
    fn sort(vector: &mut Vec<T>) {
        if vector.len() == 0 {
            return;
        }
        Self::merge_sort(vector, 0, vector.len() - 1)
    }
}

impl MergeSort {
    fn merge_sort<T>(vector: &mut Vec<T>, start: usize, end: usize) where T : PartialOrd {
        if end - start == 0 {
            return;
        }

        let mid = (end + start) / 2;
        Self::merge_sort(vector, start, mid);
        Self::merge_sort(vector, mid + 1, end);

        let mut i = start;
        let mut j = mid + 1;
        while i <= j && j <= end {
            if vector[i] > vector[j] {
                let k = vector.remove(j);
                vector.insert(i, k);

                j += 1;
            }
            i += 1;
        }
    }
}