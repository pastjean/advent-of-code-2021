pub fn p1(input: &str) -> usize {
    0
}

pub fn p2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#""#;
    const PROBLEM_INPUT: &str = include_str!("inputs/d11.txt");

    const EXAMPLE_P1_ANSWER: usize = 0;
    const EXAMPLE_P2_ANSWER: usize = 0;
    const PROBLEM_P1_ANSWER: usize = 0;
    const PROBLEM_P2_ANSWER: usize = 0;

    #[test]
    fn test_example_p1() {
        assert_eq!(p1(EXAMPLE_INPUT), EXAMPLE_P1_ANSWER);
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(p2(EXAMPLE_INPUT), EXAMPLE_P2_ANSWER);
    }

    #[test]
    fn test_problem_p1() {
        assert_eq!(p1(PROBLEM_INPUT), PROBLEM_P1_ANSWER);
    }

    #[test]
    fn test_problem_p2() {
        assert_eq!(p2(PROBLEM_INPUT), PROBLEM_P2_ANSWER);
    }
}
