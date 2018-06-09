use std::cmp;

/// Levenshtein edit distance
pub fn distance(left: &str, right: &str) -> usize {
    let mut matrix = vec![vec![0; right.len() + 1]; left.len() + 1];

    for row in 0..left.len() {
        matrix[row][0] = row;
    }

    for column in 0..right.len() {
        matrix[0][column] = column;
    }

    for (row, left_char) in left.chars().enumerate() {
        for (column, right_char) in right.chars().enumerate() {
            let deletion = matrix[row][column + 1] + 1;
            let insertion = matrix[row + 1][column] + 1;
            let substitution = matrix[row][column] + if left_char == right_char { 0 } else { 1 };
            matrix[row + 1][column + 1] = min(deletion, insertion, substitution);
        }
    }

    return matrix[left.len()][right.len()];
}

#[inline]
fn min(x: usize, y: usize, z: usize) -> usize {
    cmp::min(x, cmp::min(y, z))
}

#[cfg(test)]
mod tests {
    #[test]
    fn bla() {
        let edit_distance = super::distance("hello", "world");
        let expected_distance = 4;
        assert_eq!(edit_distance, expected_distance);
    }
}
