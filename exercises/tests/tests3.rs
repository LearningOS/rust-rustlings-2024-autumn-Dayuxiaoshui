pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        // Check if is_even returns true for an even number
        assert!(is_even(4));  // 4 is an even number
    }

    #[test]
    fn is_false_when_odd() {
        // Check if is_even returns false for an odd number
        assert!(!is_even(5));  // 5 is an odd number, so is_even(5) should be false
    }
}
