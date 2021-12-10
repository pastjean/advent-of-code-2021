use std::collections::HashSet;

type Bit = bool;
type Word = Vec<Bit>;
type Words = HashSet<Word>;
type Number = u32;

pub fn day3_p1(data: &str) -> u32 {
    let words: Words = data
        .lines()
        .map(|line| line.chars().map(|c| c == '1').collect())
        .collect();
    let width = words.iter().next().unwrap().len();

    let gamma = (0..width).fold(0, |gamma, position| {
        let ones = words.iter().filter(|word| word[position]).count();
        let zeros = words.len() - ones;
        let majority_value = ones >= zeros;

        (gamma << 1) | majority_value as Number
    });

    let mask = (1 << width) - 1;
    let epsilon = mask - gamma;

    gamma * epsilon
}

pub fn day3_p2(data: &str) -> u32 {
    let words: Words = data
        .lines()
        .map(|line| line.chars().map(|c| c == '1').collect())
        .collect();

    let oxygen = shrink(words.clone(), true);
    let co2 = shrink(words, false);

    oxygen * co2
}

fn shrink(mut words: Words, retain_majority: bool) -> Number {
    let width = words.iter().next().unwrap().len();

    for position in 0..width {
        let ones = words.iter().filter(|word| word[position]).count();
        let zeros = words.len() - ones;
        let majority_value = ones >= zeros;
        let retain_value = majority_value == retain_majority;

        words.retain(|word| word[position] == retain_value);

        if words.len() == 1 {
            let word = words.iter().next().unwrap();
            // Convert bits to number.
            return word.iter().fold(0, |x, bit| (x << 1) | *bit as Number);
        }
    }

    panic!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#;

    #[test]
    fn test_d3_p1_example() {
        assert_eq!(day3_p1(EXAMPLE_DATA), 198)
    }

    #[test]
    fn test_d3_p2_example() {
        assert_eq!(day3_p2(EXAMPLE_DATA), 230)
    }

    // tests here
    #[test]
    fn test_d3_p1() {
        let input = include_str!("inputs/d3.txt");
        assert_eq!(day3_p1(input), 2498354)
    }

    #[test]
    fn test_d3_p2() {
        let input = include_str!("inputs/d3.txt");
        assert_eq!(day3_p2(input), 3277956)
    }
}
