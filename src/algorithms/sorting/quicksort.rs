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
    let mut cursor = (start + end) / 2;
    while left < right {
        while left < cursor && list[left] <= list[cursor] { left += 1; }
        while right > cursor && list[right] >= list[cursor] { right -= 1; }
        if left < right {
            list.swap(left, right);
            cursor = if cursor == left { left } else if cursor == right { right } else { cursor };
        }
    }
    quick_sort(list, start, cursor);
    quick_sort(list, cursor + 1, end);
}
