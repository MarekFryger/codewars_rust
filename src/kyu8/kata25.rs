fn duty_free(price: i32, discount: i32, holiday_cost: i32) -> i32 {
    ((holiday_cost as f64 / price as f64) / (discount as f64 / 100 as f64)) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(duty_free(10, 10, 500), 500);
        assert_eq!(duty_free(12, 50, 1000), 166);
        assert_eq!(duty_free(17, 10, 500), 294);
    }
}
