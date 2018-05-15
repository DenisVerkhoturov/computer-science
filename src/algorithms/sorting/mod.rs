pub mod bubble;

#[cfg(test)]
mod tests {
    macro_rules! sorting_test {
        ($($name:ident: $algorithm:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let sort = $algorithm;
                    let elements = vec![23, 4, 16, 15, 8, 42];
                    let sorted = sort(&elements);
                    let expected = vec![4, 8, 15, 16, 23, 42];
                    assert_eq!(sorted, expected);
                }
            )*
        }
    }

    sorting_test! {
        bubble_sort: super::bubble::sort,
    }
}
