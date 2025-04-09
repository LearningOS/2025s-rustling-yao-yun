/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: PartialOrd>(array: &mut [T]) { // qsort 
    if array.len() <= 1 { return; }

    // partitioning 
    let mut l = 1;
    let mut r = array.len() - 1;

    loop {
        while l <= r && array[l] <= array[0] { l += 1; }
        while l <= r && array[r] > array[0] { r -= 1; }
        if l > r { break; }
        array.swap(l, r);
    }

    // move the pivot value to partition edge 
    array.swap(0, r);

    // sort  recursively 
    let (left, right) = array.split_at_mut(r);
    sort(left); 
    sort(&mut right[1..]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
