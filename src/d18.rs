use serde::Deserialize;
use std::ops::Add;

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum SnailNumber {
    Literal(i32),
    Pair(Box<Self>, Box<Self>),
}

impl SnailNumber {
    pub fn magnitude(&self) -> i32 {
        match self {
            Self::Literal(v) => *v,
            Self::Pair(a, b) => 3 * a.magnitude() + 2 * b.magnitude(),
        }
    }

    fn explode(&mut self, depth: usize) -> Option<(i32, i32)> {
        match self {
            SnailNumber::Literal(_) => None,
            SnailNumber::Pair(l, r) => match depth {
                4 => {
                    let a = match **l {
                        SnailNumber::Literal(val) => val,
                        _ => unreachable!(),
                    };
                    let b = match **r {
                        SnailNumber::Literal(val) => val,
                        _ => unreachable!(),
                    };
                    *self = SnailNumber::Literal(0);

                    Some((a, b))
                }
                _ => {
                    if let Some((a, b)) = l.explode(depth + 1) {
                        r.absorb(true, b);
                        return Some((a, 0));
                    }
                    if let Some((a, b)) = r.explode(depth + 1) {
                        r.absorb(false, a);
                        return Some((0, b));
                    }

                    None
                }
            },
        }
    }

    fn absorb(&mut self, from_left: bool, val: i32) {
        match self {
            SnailNumber::Literal(prev) => *prev += val,
            SnailNumber::Pair(l, r) => match from_left {
                true => l.absorb(from_left, val),
                false => r.absorb(from_left, val),
            },
        }
    }

    fn split(&mut self) -> Option<()> {
        match self {
            SnailNumber::Literal(val) => {
                if *val >= 10 {
                    *self = SnailNumber::Pair(
                        Box::new(SnailNumber::Literal((*val as f32 / 2.0).floor() as i32)),
                        Box::new(SnailNumber::Literal((*val as f32 / 2.0).ceil() as i32)),
                    );
                    Some(())
                } else {
                    None
                }
            }
            SnailNumber::Pair(left, right) => {
                if left.split().is_some() {
                    return Some(());
                }
                if right.split().is_some() {
                    return Some(());
                }
                None
            }
        }
    }
}

impl Add for SnailNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut new_pair = Self::Pair(Box::new(self), Box::new(rhs));

        // Reduce
        loop {
            if new_pair.explode(0).is_some() {
                continue;
            }

            if new_pair.split().is_some() {
                continue;
            }

            break;
        }

        new_pair
    }
}

pub fn p1(input: &str) -> i32 {
    let v: Vec<_> = input
        .trim()
        .lines()
        .map(|l| serde_json::from_str::<SnailNumber>(l).unwrap())
        .collect();

    v.iter()
        .skip(1)
        .fold(v[0].clone(), |acc, nxt| acc + nxt.clone())
        .magnitude()
}

pub fn p2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROBLEM_INPUT: &str = include_str!("inputs/d18.txt");

    const PROBLEM_P1_ANSWER: i32 = 0;
    const PROBLEM_P2_ANSWER: usize = 0;

    #[test]
    fn test_problem_p1() {
        assert_eq!(p1(PROBLEM_INPUT), PROBLEM_P1_ANSWER);
    }

    #[test]
    fn test_problem_p2() {
        assert_eq!(p2(PROBLEM_INPUT), PROBLEM_P2_ANSWER);
    }
}
