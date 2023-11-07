fn fake_bin(s: &str) -> String {
    let mut res = String::new();
    for ele in s.chars() {
        if ( u8::from_str_radix(&String::from(ele),16)).unwrap() >= 5{
            res.push('1');
        } else {
            res.push('0');
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::fake_bin;

    #[test]
    fn basic_tests() {
        assert_eq!(fake_bin("45385593107843568"), "01011110001100111");
        assert_eq!(fake_bin("509321967506747"), "101000111101101");
        assert_eq!(
            fake_bin("366058562030849490134388085"),
            "011011110000101010000011011"
        );
        assert_eq!(fake_bin("15889923"), "01111100");
        assert_eq!(fake_bin("800857237867"), "100111001111");
    }
}
