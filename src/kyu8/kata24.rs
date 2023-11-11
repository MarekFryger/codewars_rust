fn feast(beast: &str, dish: &str) -> bool {
    beast.chars().last().unwrap() == dish.chars().last().unwrap()
        && beast.chars().nth(0).unwrap() == dish.chars().nth(0).unwrap()
}

#[test]
fn sample_test_cases() {
    assert_eq!(feast("great blue heron", "garlic naan"), true);
    assert_eq!(feast("chickadee", "chocolate cake"), true);
    assert_eq!(feast("brown bear", "bear claw"), false);
}
