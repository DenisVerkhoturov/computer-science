/// Quicksort
pub fn sort<T: Ord + Clone>(list: &Vec<T>) -> Vec<T> {
    let mut sorted = list.clone();
    quick_sort(&mut sorted, 0, list.len() - 1);
    return sorted;
}

fn quick_sort<T: Ord + Clone>(list: &mut Vec<T>, start: usize, end: usize) {
    if start >= end { return; }
    let mut left = start;
    let mut right = end;
    let mut pivot = (start + end) / 2;
    while left < right {
        while left < pivot && list[left] <= list[pivot] { left += 1; }
        while right > pivot && list[right] >= list[pivot] { right -= 1; }
        if left < right {
            list.swap(left, right);
            pivot = if left == pivot { right } else if right == pivot { left } else { pivot };
        }
    }
    quick_sort(list, start, pivot);
    quick_sort(list, pivot + 1, end);
}
