use crate::sort_algorithms::sort::Sort;

pub struct BubbleSort;

impl<T> Sort<T> for BubbleSort where T : PartialOrd{
    fn sort(vector: &mut Vec<T>) {
        for _ in 0..vector.len() {
            let mut sorted = true;
            for j in 0..vector.len() - 1 {
                if vector[j + 1] < vector[j] {
                    vector.swap(j + 1, j);
                    sorted = false;
                }
            }
            if sorted {
                break;
            }
        }
    }
}