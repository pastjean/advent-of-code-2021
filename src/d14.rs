use std::collections::HashMap;

use itertools::{Itertools, MinMaxResult};

type Seq = HashMap<(char, char), usize>;
type Rules = HashMap<(char, char), char>;

fn parse_input(input: &str) -> (Rules, Seq) {
    let (start_seq, pairs) = input.trim().split_once("\n\n").unwrap();

    let rules = pairs
        .lines()
        .filter_map(|l| l.split(" -> ").collect_tuple::<(&str, &str)>())
        .map(|(lhs, rhs)| {
            (
                lhs.chars().collect_tuple::<(char, char)>().unwrap(),
                rhs.chars().next().unwrap(),
            )
        })
        .collect();

    let seq = format!("_{}_", start_seq).chars().tuple_windows().counts();

    (rules, seq)
}

pub fn grow_n_count(input: &str, i: usize) -> usize {
    let (rules, init_seq) = parse_input(input);

    let seq = (0..i).fold(init_seq, |seq, _i| {
        let mut new_seq = HashMap::new();

        for ((c1, c2), count) in seq {
            if let Some(&c_mid) = rules.get(&(c1, c2)) {
                *new_seq.entry((c1, c_mid)).or_insert(0) += count;
                *new_seq.entry((c_mid, c2)).or_insert(0) += count;
            } else {
                *new_seq.entry((c1, c2)).or_insert(0) += count;
            }
        }
        new_seq
    });

    let mut counts = HashMap::new();
    for ((c1, c2), count) in seq {
        *counts.entry(c1).or_insert(0) += count;
        *counts.entry(c2).or_insert(0) += count;
    }
    counts.remove(&'_');

    match counts.values().minmax() {
        MinMaxResult::MinMax(min, max) => (*max - *min) / 2,
        _ => 0,
    }
}

pub fn p1(input: &str) -> usize {
    grow_n_count(input, 10)
}

pub fn p2(input: &str) -> usize {
    grow_n_count(input, 40)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
"#;
    const PROBLEM_INPUT: &str = include_str!("inputs/d14.txt");

    const EXAMPLE_P1_ANSWER: usize = 1588;
    const EXAMPLE_P2_ANSWER: usize = 2188189693529;
    const PROBLEM_P1_ANSWER: usize = 2657;
    const PROBLEM_P2_ANSWER: usize = 2911561572630;

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
