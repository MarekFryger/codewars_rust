fn hello(name: &str) -> String {
    if name.len() > 0 {
        return format!(
            "Hello, {}{}!",
            String::from(name.chars().next().unwrap().to_uppercase().to_string()),
            String::from(name.to_lowercase().chars().skip(1).collect::<String>())
        );
    }
    return "Hello, World!".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(hello("johN"), "Hello, John!");
        assert_eq!(hello("alice"), "Hello, Alice!");
        assert_eq!(hello(""), "Hello, World!");
    }
}
