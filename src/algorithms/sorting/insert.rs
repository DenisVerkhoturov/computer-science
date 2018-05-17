/// Insert sort
pub fn sort<T: Ord + Clone>(list: &Vec<T>) -> Vec<T> {
    let mut sorted = list.clone();
    for cursor in 1..sorted.len() {
        let value = sorted[cursor].clone();
        let mut place = cursor;
        while place > 0 && sorted[place - 1] > value {
            sorted[place] = sorted[place - 1].clone();
            place -= 1;
        }
        sorted[place] = value;
    }
    return sorted;
}
