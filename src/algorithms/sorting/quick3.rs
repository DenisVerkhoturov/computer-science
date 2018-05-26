/// Three way quick sort
pub fn sort<T: Ord + Clone>(list: &Vec<T>) -> Vec<T> {
    three_way_quick_sort(list.clone())
}

fn three_way_quick_sort<T: Ord + Clone>(list: Vec<T>) -> Vec<T> {
    if list.is_empty() { return list; }
    let pivot = list[list.len() / 2].clone();
    let mut accumulator: (Vec<T>, Vec<T>, Vec<T>) = (Default::default(), Default::default(), Default::default());
    let (less, equal, greater) = list.into_iter().fold(
        accumulator,
        |(mut less, mut equal, mut greater): (Vec<T>, Vec<T>, Vec<T>), element: T| {
            if element < pivot {
                less.push(element);
            } else if element == pivot {
                equal.push(element);
            } else {
                greater.push(element);
            };
            return (less, equal, greater);
        },
    );
    [three_way_quick_sort(less), equal, three_way_quick_sort(greater)].concat()
}
