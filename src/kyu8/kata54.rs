fn update_light(current: &str) -> String {
    match current {
        "yellow" => "red".to_string(),
        "green" => "yellow".to_string(),
        _ => "green".to_string()
    }
}

#[test]
fn basic_test() {
    assert_eq!(update_light("green"), "yellow");
    assert_eq!(update_light("yellow"), "red");
    assert_eq!(update_light("red"), "green");
}
