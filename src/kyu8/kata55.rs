fn quadrant(x: i32, y: i32) -> i32 {
    if x.is_negative() && y.is_positive() {
        return 2;
    }
    if x.is_negative() && y.is_negative() {
        return 3;
    }
    if x.is_positive() && y.is_negative() {
        return 4;
    }

    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_tests() {
        assert_eq!(quadrant(1, 2), 1);
        assert_eq!(quadrant(3, 5), 1);
        assert_eq!(quadrant(-10, 100), 2);
        assert_eq!(quadrant(-1, -9), 3);
        assert_eq!(quadrant(19, -56), 4);
    }
}
