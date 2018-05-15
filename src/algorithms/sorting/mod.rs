pub mod bubble;

#[cfg(test)]
mod tests {
    use super::bubble;

    #[test]
    fn bubble_sort() {
        let elements = vec![23, 4, 16, 15, 8, 42];
        let sorted = bubble::sort(&elements);
        let expected = vec![4, 8, 15, 16, 23, 42];
        assert_eq!(sorted, expected);
    }
}
