/// Gnome sort
pub fn sort<T: Ord + Clone>(list: &Vec<T>) -> Vec<T> {
    let mut sorted = list.clone();
    let mut i = 1;
    while i < list.len() {
        if i == 0 || sorted[i - 1] <= sorted[i] {
            i += 1;
        } else {
            sorted.swap(i - 1, i);
            i -= 1;
        }
    }
    return sorted;
}
