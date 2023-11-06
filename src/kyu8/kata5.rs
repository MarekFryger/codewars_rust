fn neutralise(s1: &str, s2: &str) -> String {
    let mut res = String::new();
    for (char1, char2) in s1.chars().zip(s2.chars()) {
        if char1 == char2 && char1 == '+' {
            res.push('+');
        } else if char1 == char2 && char1 == '-' {
            res.push('-');
        } else {
            res.push('0');
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::neutralise;

    fn dotest(s1: &str, s2: &str, expected: &str) {
        let actual = neutralise(s1, s2);
        assert!(
            actual == expected,
            "With s1 = \"{s1}\" and s2 = \"{s2}\"\nExpected {expected} but got {actual}"
        )
    }

    #[test]
    fn fixed_tests() {
        dotest("--++--", "++--++", "000000");
        dotest("-+-+-+", "-+-+-+", "-+-+-+");
        dotest("-++-", "-+-+", "-+00");
        dotest("--++", "++++", "00++");
        dotest("+++--+---", "++----++-", "++0--000-");
        dotest("-----", "-----", "-----");
        dotest("-+", "++", "0+");
        dotest("--", "-+", "-0");
        dotest("-++", "+--", "000");
        dotest("++-++--++-", "-+++-++-++", "0+0+0000+0");
        dotest("-++-+-++-", "+-++++---", "00+0+000-");
        dotest("---++-+--", "-+++--++-", "-00+0-+0-");
        dotest("+-----+++-", "--+-+-++--", "0-0-0-++0-");
        dotest("+-----+-", "---++-++", "0--00-+0");
        dotest("-+--+-+---", "-+--+-+-+-", "-+--+-+-0-");
        dotest("+-+", "-++", "00+");
        dotest("-++", "-+-", "-+0");
        dotest("---+", "-+++", "-00+");
        dotest("+--", "+--", "+--");
        dotest("--+++-+-", "+++++---", "00+++-0-");
    }
}
