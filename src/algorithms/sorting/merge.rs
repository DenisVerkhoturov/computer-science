pub mod top_down {
    pub fn sort<T: Ord + Clone>(list: &Vec<T>) -> Vec<T> {
        return split_and_merge(list);
    }

    fn split_and_merge<T: Ord + Clone>(list: &[T]) -> Vec<T> {
        if list.len() > 1 {
            let mid = list.len() / 2;
            let left = split_and_merge(&list[..mid]);
            let right = split_and_merge(&list[mid..]);
            return super::merge(&left, &right);
        } else {
            return list.to_vec();
        }
    }
}

pub mod bottom_up {
    use std::cmp::min;

    pub fn sort<T: Ord + Clone>(list: &Vec<T>) -> Vec<T> {
        let mut sorted = list.clone();
        let mut range = 1;
        while range <= list.len() {
            range *= 2;
            let mut start = 0;
            while start < list.len() {
                let end = min(start + range, list.len());
                let mid = min(start + range / 2, list.len());
                let merged = super::merge(&sorted[start..mid], &sorted[mid..end]);
                sorted.splice(start..end, merged);
                start += range;
            }
        }
        return sorted;
    }
}

fn merge<T: Ord + Clone>(left: &[T], right: &[T]) -> Vec<T> {
    let count = left.len() + right.len();
    let mut merged: Vec<T> = Vec::with_capacity(count);
    let mut left_index = 0;
    let mut right_index = 0;
    while merged.len() < count {
        if (left_index < left.len()) && (right_index >= right.len() || left[left_index] <= right[right_index]) {
            merged.push(left[left_index].clone());
            left_index += 1;
        } else {
            merged.push(right[right_index].clone());
            right_index += 1;
        };
    }
    return merged;
}
