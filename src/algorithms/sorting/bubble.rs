/// Bubble sort
pub fn sort<T: Ord + Clone>(list: &Vec<T>) -> Vec<T> {
    let mut sorted = list.clone();
    for left in 0..sorted.len() {
        for right in 0..sorted.len() {
            if sorted[left] < sorted[right] {
                sorted.swap(left, right)
            }
        }
    }
    return sorted;
}
