pub mod bubble;
pub mod gnome;
pub mod quicksort;
pub mod insert;
pub mod select;
pub mod merge;
pub mod quick3;

macro_rules! sorting_test {
    ($($name:ident: $algorithm:expr,)*) => {
        $(
            #[cfg(test)]
            mod $name {
                #[test]
                fn should_sort_sorted() {
                    let expected = vec![4, 8, 15, 16, 23, 42];
                    let elements = vec![4, 8, 15, 16, 23, 42];
                    let sorted = $algorithm(&elements);
                    assert_eq!(sorted, expected);
                }

                #[test]
                fn should_sort_random_sorted() {
                    let expected = vec![4, 8, 15, 16, 23, 42];
                    let elements = vec![23, 4, 16, 15, 8, 42];
                    let sorted = $algorithm(&elements);
                    assert_eq!(sorted, expected);
                }

                #[test]
                fn should_sort_nearly_sorted() {
                    let expected = vec![4, 8, 15, 16, 23, 42];
                    let elements = vec![8, 4, 15, 23, 16, 42];
                    let sorted = $algorithm(&elements);
                    assert_eq!(sorted, expected);
                }

                #[test]
                fn should_sort_reversed() {
                    let expected = vec![4, 8, 15, 16, 23, 42];
                    let elements = vec![42, 23, 16, 15, 8, 4];
                    let sorted = $algorithm(&elements);
                    assert_eq!(sorted, expected);
                }

                #[test]
                fn should_sort_few_unique() {
                    let expected = vec![8, 8, 23, 23, 42, 42];
                    let elements = vec![42, 23, 8, 23, 8, 42];
                    let sorted = $algorithm(&elements);
                    assert_eq!(sorted, expected);
                }
            }
        )*
    }
}

sorting_test! {
    bubble_sort: super::bubble::sort,
    gnome_sort: super::gnome::sort,
    quick_sort: super::quicksort::sort,
    three_way_quick_sort: super::quick3::sort,
    insert_sort: super::insert::sort,
    select_sort: super::select::sort,
    merge_top_down_sort: super::merge::top_down::sort,
    merge_bottom_up_sort: super::merge::bottom_up::sort,
}
