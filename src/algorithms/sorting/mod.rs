pub mod bubble;
pub mod quicksort;
pub mod insert;

#[cfg(test)]
mod tests {
    macro_rules! sorting_test {
        ($($name:ident: $algorithm:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let expected = vec![4, 8, 15, 16, 23, 42];
                    let elements = vec![23, 4, 16, 15, 8, 42];
                    let sorted = $algorithm(&elements);
                    assert_eq!(sorted, expected);
                }
            )*
        }
    }

    sorting_test! {
        bubble_sort: super::bubble::sort,
        quick_sort: super::quicksort::sort,
        insert_sort: super::insert::sort,
    }
}
