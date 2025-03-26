/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: PartialOrd + Copy>(array: &mut [T]) {
    quick_sort(array, 0, array.len() - 1);
}

fn quick_sort<T: PartialOrd + Copy>(array: &mut [T], left: usize, right: usize) {
    if left >= right {
        return;
    }
    let pivot = array[left];
    let mut index = left + 1;
    for i in left..=right {
        if array[i] < pivot {
            array.swap(i, index);
            index += 1;
        }
    }
    array.swap(left, index - 1);
    quick_sort(array, left, index - 1);
    quick_sort(array, index, right);
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
