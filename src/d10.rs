fn illegal_char(line: &str) -> Option<char> {
    let mut close = vec![];
    for c in line.chars() {
        match c {
            '(' => close.push(')'),
            '[' => close.push(']'),
            '{' => close.push('}'),
            '<' => close.push('>'),
            _ => {
                if close.pop() != Some(c) {
                    return Some(c);
                }
            }
        }
    }
    None
}

fn score_corrupted(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

fn completion_chars(line: &str) -> Option<String> {
    let mut close = vec![];
    for c in line.chars() {
        match c {
            '(' => close.push(')'),
            '[' => close.push(']'),
            '{' => close.push('}'),
            '<' => close.push('>'),
            _ => {
                if close.pop() != Some(c) {
                    return None;
                }
            }
        }
    }
    Some(close.into_iter().rev().collect())
}

fn score_completion(s: String) -> usize {
    s.chars().fold(0, |acc, c| {
        let score = match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => unreachable!(),
        };

        acc * 5 + score
    })
}

pub fn p1(input: &str) -> usize {
    input
        .lines()
        .filter_map(|l| illegal_char(l))
        .map(score_corrupted)
        .sum()
}

pub fn p2(input: &str) -> usize {
    let mut scores = input
        .lines()
        .filter_map(|l| completion_chars(l))
        .map(score_completion)
        .collect::<Vec<usize>>();

    scores.sort_unstable();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#;

    #[test]
    fn test_p1_example() {
        assert_eq!(p1(EXAMPLE), 26397);
    }

    #[test]
    fn test_p1() {
        let input = include_str!("inputs/d10.txt");
        assert_eq!(p1(input), 392097);
    }

    #[test]
    fn test_p2_example() {
        let input = EXAMPLE;
        assert_eq!(p2(input), 288957);
    }

    #[test]
    fn test_p2() {
        let input = include_str!("inputs/d10.txt");
        assert_eq!(p2(input), 4263222782);
    }
}
