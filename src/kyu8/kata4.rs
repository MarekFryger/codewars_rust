fn greet(name: &str) -> String {
    let mut res = String::from("Hello, ");
    res.push_str(name);
    res.push_str(" how are you doing today?");
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(greet("Ryan"), "Hello, Ryan how are you doing today?");
        assert_eq!(
            greet("Shingles"),
            "Hello, Shingles how are you doing today?"
        );
    }
}
