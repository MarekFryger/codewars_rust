use std::ops::Add;

fn two_sort(arr: &[&str]) -> String {
    let mut res = String::new();
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort_by(|a, b| a.cmp(b));
    for ch in sorted_arr.get(0).unwrap().chars() {
        res.push(ch);
        res = res.clone().add("***");
    }
    res[..res.len() - 3].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test_cases() {
        assert_eq!(
            two_sort(&[
                "bitcoin", "take", "over", "the", "world", "maybe", "who", "knows", "perhaps"
            ]),
            "b***i***t***c***o***i***n"
        );
        assert_eq!(
            two_sort(&[
                "turns", "out", "random", "test", "cases", "are", "easier", "than", "writing",
                "out", "basic", "ones"
            ]),
            "a***r***e"
        );
    }
}
