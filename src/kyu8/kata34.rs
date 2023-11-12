fn flip(dir: char, cubes: &[u32]) -> Vec<u32> {
    let mut vec = Vec::from(cubes);
    if dir == 'R' {
        vec.sort();
        return vec.iter().cloned().collect();
    } else {
        vec.sort();
        vec.reverse();
        return vec.iter().cloned().collect();
    }
}

#[cfg(test)]
mod tests {
    use super::flip;

    #[test]
    fn sample_tests() {
        assert_eq!(flip('R', &vec![3, 2, 1, 2]), vec![1, 2, 2, 3]);
        assert_eq!(flip('L', &vec![1, 4, 5, 3, 5]), vec![5, 5, 4, 3, 1]);
    }
}
