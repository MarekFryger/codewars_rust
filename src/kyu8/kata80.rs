fn usdcny(usd: u16) -> String {
    format!("{:.2} Chinese Yuan", usd as f32 * 6.75)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(usdcny(15), "101.25 Chinese Yuan");
        assert_eq!(usdcny(465), "3138.75 Chinese Yuan");
        assert_eq!(usdcny(100), "675.00 Chinese Yuan");
    }
}
