fn powers_of_two(n: u8) -> Vec<u128> {
    let mut res: Vec<u128> = Vec::new();
    for i in 0..=n {
        let mut tmp: u128 = 1;
        for j in 1..=i {
            tmp = 2 * tmp;
        }
        res.push(tmp);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::powers_of_two;

    fn dotest(n: u8, expected: &[u128]) {
        let actual = powers_of_two(n);
        assert!(
            actual == expected,
            "With n = {n}\nExpected {expected:?}\nBut got {actual:?}"
        )
    }

    #[test]
    fn fixed_tests() {
        dotest(0, &[1]);
        dotest(1, &[1, 2]);
        dotest(4, &[1, 2, 4, 8, 16]);
    }
}
