use std::collections::HashMap;

const START: usize = 0;
const END: usize = 1;

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    let id = input.trim().lines().flat_map(|line| line.split('-')).fold(
        HashMap::from([("start", START), ("end", END)]),
        |mut acc, cave| {
            let next_id = acc.len();
            acc.entry(cave).or_insert(next_id);
            acc
        },
    );

    let mut caves = input
        .lines()
        .filter_map(|line| line.split_once('-').map(|(l, r)| (id[l], id[r])))
        .flat_map(|(l, r)| [(l, r), (r, l)])
        .filter(|(l, r)| l != &END && r != &START)
        .fold(vec![vec![]; id.len()], |mut caves, (l, r)| {
            caves[l].push(r);
            caves
        });

    // flatten big caves
    id.iter()
        .filter(|&(name, _)| name.chars().any(|c| c.is_uppercase()))
        .for_each(|(_, &big)| {
            let smalls = caves[big].clone();
            caves.iter_mut().for_each(|nexts| {
                if let Some(i) = nexts.iter().position(|&next| next == big) {
                    nexts.splice(i..i + 1, smalls.iter().copied());
                }
            });
        });
    caves
}

fn dfs(caves: &[Vec<usize>], visited: &mut Vec<bool>, curr: usize, extra: usize) -> usize {
    caves[curr].iter().fold(0, |acc, &next| match next {
        END => acc + 1,
        _ => match visited[next] {
            true => match extra {
                0 => acc,
                _ => acc + dfs(caves, visited, next, extra - 1),
            },
            _ => {
                visited[next] = true;
                let paths = dfs(caves, visited, next, extra);
                visited[next] = false;
                acc + paths
            }
        },
    })
}

pub fn p1(input: &str) -> usize {
    let caves = parse_input(input);

    println!("{:?}", caves);
    dfs(&caves, &mut vec![false; caves.len()], START, 0)
}

pub fn p2(input: &str) -> usize {
    let caves = parse_input(input);
    dfs(&caves, &mut vec![false; caves.len()], START, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#;
    const PROBLEM_INPUT: &str = include_str!("inputs/d12.txt");

    const EXAMPLE_P1_ANSWER: usize = 10;
    const EXAMPLE_P2_ANSWER: usize = 36;
    const PROBLEM_P1_ANSWER: usize = 3708;
    const PROBLEM_P2_ANSWER: usize = 93858;

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
