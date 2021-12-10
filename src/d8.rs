use std::collections::HashSet;

pub fn p1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .flat_map(|line| line.split(" | ").nth(1).unwrap().split(' '))
        .filter(|&v| matches!(v.len(), 2 | 3 | 4 | 7))
        .count()
}

// 1 ..c..f. => len = 2
// 7 a.c..f. => len = 3
// 4 .bcd.f. => len = 4
// 8 abcdefg => len = 7
// 9 abcd.fg => len = 6 & 4.diff(9) == 0
// 0 abc.efg => len = 6 & 1.diff(0) == 0
// 6 ab.defg => len = 6
// 3 a.cd.fg => len = 5 & 1.diff(3) == 0
// 5 ab.d.fg => len = 5 & 4.diff(5) == 1
// 2 a.cde.g => len = 5 & 4.diff(2) == 2

pub fn p2(input: &str) -> i32 {
    let mut total = 0;

    for line in input.trim().lines() {
        let mut sides = line.split(" | ").map(|side| {
            side.split(' ')
                .map(|s| s.chars().collect::<HashSet<char>>())
        });

        let possible_patterns = sides.next().unwrap();
        let display_side = sides.next().unwrap();

        let mut mapping: [HashSet<char>; 10] = Default::default();
        let mut patterns_round_2: Vec<_> = Default::default();

        // Decoding level 1 - Easy to decode patterns 1,4,7,8
        for pattern in possible_patterns {
            match pattern.len() {
                2 => mapping[1] = pattern,
                3 => mapping[7] = pattern,
                4 => mapping[4] = pattern,
                7 => mapping[8] = pattern,
                _ => patterns_round_2.push(pattern),
            }
        }

        // Decoding level 2 - depends on level 1
        for pattern in patterns_round_2 {
            match (pattern.len(), pattern) {
                // Order is important
                (5, i) if mapping[1].difference(&i).count() == 0 => mapping[3] = i,
                (5, i) if mapping[4].difference(&i).count() == 1 => mapping[5] = i,
                (5, i) => mapping[2] = i,
                (6, i) if mapping[4].difference(&i).count() == 0 => mapping[9] = i,
                (6, i) if mapping[1].difference(&i).count() == 0 => mapping[0] = i,
                (6, i) => mapping[6] = i,
                _ => unreachable!(),
            }
        }

        let number = display_side.fold(0, |number, pattern| {
            number * 10 + mapping.iter().position(|x| pattern.eq(x)).unwrap() as i32
        });

        total += number;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_example() {
        let input = include_str!("inputs/d8_example.txt");
        assert_eq!(p1(input), 26);
    }

    #[test]
    fn test_p1() {
        let input = include_str!("inputs/d8.txt");
        assert_eq!(p1(input), 278);
    }

    #[test]
    fn test_p2_example() {
        let input = include_str!("inputs/d8_example.txt");
        assert_eq!(p2(input), 61229);
    }

    #[test]
    fn test_p2() {
        let input = include_str!("inputs/d8.txt");
        assert_eq!(p2(input), 986179);
    }
}
