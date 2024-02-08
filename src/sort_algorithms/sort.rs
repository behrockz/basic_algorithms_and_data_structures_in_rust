pub trait Sort<T> where T : PartialOrd {
    fn sort(vector: &mut Vec<T>) ;
}


#[cfg(test)]
mod tests {
    use crate::sort_algorithms::bubble_sort::BubbleSort;
    use crate::sort_algorithms::merge_sort::MergeSort;
    use crate::sort_algorithms::quick_sort::QuickSort;
    use crate::sort_algorithms::selection_sort::SelectionSort;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn initialise_vectors(_vec1: &mut Vec<i32>,
                          vec2: &mut Vec<i32>,
                          vec3: &mut Vec<i32>,
                          vec4: &mut Vec<i32>,
                          vec5: &mut Vec<i32>,
                          vec6: &mut Vec<i32>){
        vec2.push(4);

        vec3.push(4);
        vec3.push(2);

        vec4.push(4);
        vec4.push(2);
        vec4.push(4);
        vec4.push(6);
        vec4.push(1);
        vec4.push(9);

        vec5.push(4);
        vec5.push(2);
        vec5.push(4);
        vec5.push(6);
        vec5.push(1);
        vec5.push(9);
        vec5.push(3);


        vec6.push(4);
        vec6.push(2);
        vec6.push(4);
        vec6.push(6);
        vec6.push(1);
        vec6.push(9);
        vec6.push(3);
        vec6.push(6);
        vec6.push(1);
        vec6.push(12);
        vec6.push(2);
        vec6.push(17);
        vec6.push(1);
        vec6.push(0);
    }

    fn verify (vector: Vec<i32>) -> bool {
        let mut prev = 0;
        for i in vector {
            if i < prev {
                return false;
            }
            prev = i;
        }
        return true;
    }
    #[test]
    fn bubble_sort() {
        let mut v1 = Vec::<i32>::new();
        let mut v2 = Vec::<i32>::new();
        let mut v3 = Vec::<i32>::new();
        let mut v4 = Vec::<i32>::new();
        let mut v5 = Vec::<i32>::new();
        let mut v6 = Vec::<i32>::new();
        initialise_vectors(&mut v1, &mut v2, &mut v3, &mut v4, &mut v5, &mut v6);

        BubbleSort::sort(&mut v1);
        BubbleSort::sort(&mut v2);
        BubbleSort::sort(&mut v3);
        BubbleSort::sort(&mut v4);
        BubbleSort::sort(&mut v5);
        BubbleSort::sort(&mut v6);

        assert!(verify(v1));
        assert!(verify(v2));
        assert!(verify(v3));
        assert!(verify(v4));
        assert!(verify(v5));
        assert!(verify(v6));
    }
    #[test]
    fn selection_sort() {
        let mut v1 = Vec::<i32>::new();
        let mut v2 = Vec::<i32>::new();
        let mut v3 = Vec::<i32>::new();
        let mut v4 = Vec::<i32>::new();
        let mut v5 = Vec::<i32>::new();
        let mut v6 = Vec::<i32>::new();
        initialise_vectors(&mut v1, &mut v2, &mut v3, &mut v4, &mut v5, &mut v6);

        SelectionSort::sort(&mut v1);
        SelectionSort::sort(&mut v2);
        SelectionSort::sort(&mut v3);
        SelectionSort::sort(&mut v4);
        SelectionSort::sort(&mut v5);
        SelectionSort::sort(&mut v6);

        assert!(verify(v1));
        assert!(verify(v2));
        assert!(verify(v3));
        assert!(verify(v4));
        assert!(verify(v5));
        assert!(verify(v6));
    }
    #[test]
    fn merge_sort() {
        let mut v1 = Vec::<i32>::new();
        let mut v2 = Vec::<i32>::new();
        let mut v3 = Vec::<i32>::new();
        let mut v4 = Vec::<i32>::new();
        let mut v5 = Vec::<i32>::new();
        let mut v6 = Vec::<i32>::new();
        initialise_vectors(&mut v1, &mut v2, &mut v3, &mut v4, &mut v5, &mut v6);

        MergeSort::sort(&mut v1);
        MergeSort::sort(&mut v2);
        MergeSort::sort(&mut v3);
        MergeSort::sort(&mut v4);
        MergeSort::sort(&mut v5);
        MergeSort::sort(&mut v6);

        assert!(verify(v1));
        assert!(verify(v2));
        assert!(verify(v3));
        assert!(verify(v4));
        assert!(verify(v5));
        assert!(verify(v6));
    }
    #[test]
    fn quick_sort() {
        let mut v1 = Vec::<i32>::new();
        let mut v2 = Vec::<i32>::new();
        let mut v3 = Vec::<i32>::new();
        let mut v4 = Vec::<i32>::new();
        let mut v5 = Vec::<i32>::new();
        let mut v6 = Vec::<i32>::new();
        initialise_vectors(&mut v1, &mut v2, &mut v3, &mut v4, &mut v5, &mut v6);

        QuickSort::sort(&mut v1);
        QuickSort::sort(&mut v2);
        QuickSort::sort(&mut v3);
        QuickSort::sort(&mut v4);
        QuickSort::sort(&mut v5);
        QuickSort::sort(&mut v6);

        assert!(verify(v1));
        assert!(verify(v2));
        assert!(verify(v3));
        assert!(verify(v4));
        assert!(verify(v5));
        assert!(verify(v6));
    }
}