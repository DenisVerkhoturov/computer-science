/// Selection sort
pub fn sort<T: Ord + Clone>(list: &Vec<T>) -> Vec<T> {
    let mut sorted = list.clone();
    for index in 0..list.len() {
        let index_of_least = index_of_least_from(&sorted, index);
        sorted.swap(index_of_least, index);
    }
    return sorted;
}

fn index_of_least_from<T: Ord + Clone>(list: &Vec<T>, start: usize) -> usize {
    let mut index_of_least = start;
    for (index, element) in list.iter().enumerate().skip(start) {
        if element < &list[index_of_least] {
            index_of_least = index;
        }
    }
    return index_of_least;
}
