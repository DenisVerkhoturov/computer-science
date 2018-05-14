pub trait Locatable {
    fn lower_bound(&self) -> i64;

    fn upper_bound(&self) -> i64;

    fn is_before(&self, another: &Locatable) -> bool {
        self.upper_bound() < another.lower_bound()
    }

    fn is_after(&self, another: &Locatable) -> bool {
        another.upper_bound() < self.lower_bound()
    }

    fn is_concurrent(&self, another: &Locatable) -> bool {
        self.contains(another.lower_bound()) || self.contains(another.upper_bound())
    }

    fn is_abutting(&self, another: &Locatable) -> bool {
        self.upper_bound() + 1 == another.lower_bound() || another.upper_bound() + 1 == self.lower_bound()
    }

    fn contains(&self, value: i64) -> bool {
        self.lower_bound() <= value && value <= self.upper_bound()
    }
}

pub struct Range {
    lower_bound: i64,
    upper_bound: i64,
}

impl Range {
    pub fn new(lower_bound: i64, upper_bound: i64) -> Self {
        if lower_bound > upper_bound {
            panic!("Lower bound of a Range should be less than or equal to upper one.")
        }
        Range { lower_bound, upper_bound }
    }
}

impl Locatable for Range {
    fn lower_bound(&self) -> i64 {
        self.lower_bound
    }

    fn upper_bound(&self) -> i64 {
        self.upper_bound
    }
}

#[cfg(test)]
mod tests {
    use super::Range;
    use super::Locatable;

    #[test]
    #[should_panic(expected = "Lower bound of a Range should be less than or equal to upper one.")]
    fn should_panic_if_lower_bound_is_greater_than_upper_one() {
        Range::new(47, 42);
    }

    #[test]
    fn should_not_panic_if_lower_bound_equals_upper_one() {
        Range::new(42, 42);
    }

    #[test]
    fn should_not_panic_if_lower_bound_is_i64_min_value_and_upper_one_is_i64_max_value() {
        Range::new(i64::min_value(), i64::max_value());
    }

    #[test]
    fn should_contain_lower_and_upper_bounds_as_elements() {
        let range = Range::new(42, 47);
        assert!(range.contains(42) && range.contains(47));
    }

    #[test]
    fn should_not_contain_an_element_if_it_is_out_of_its_bounds() {
        let range = Range::new(42, 47);
        assert!(!range.contains(15));
    }

    #[test]
    fn should_be_before_another_if_its_upper_bound_is_less_than_lower_bound_of_another_range() {
        let range = Range::new(42, 47);
        let another = Range::new(48, 52);
        assert!(range.is_before(&another));
    }
}
