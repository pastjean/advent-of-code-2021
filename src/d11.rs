#[rustfmt::skip]
const NEXT: [(isize, isize); 8] = [(0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1)];
const SIZE: usize = 10;

pub fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn flash(map: &mut Vec<Vec<u32>>, x: usize, y: usize) -> usize {
    map[y][x] = 0;
    NEXT.iter()
        .map(|(xx, yy)| ((x as isize + xx) as usize, (y as isize + yy) as usize))
        .fold(1, |acc, (x, y)| {
            match map.get_mut(y).and_then(|l| l.get_mut(x)) {
                Some(cell) if *cell > 0 => {
                    *cell += 1;
                    acc + (*cell > 9).then(|| flash(map, x, y)).unwrap_or(0)
                }
                _ => acc,
            }
        })
}

pub fn p1(input: &str) -> usize {
    let mut parsed_input = parse_input(input);

    (0..100).fold(0, |acc, _| {
        parsed_input
            .iter_mut()
            .for_each(|row| row.iter_mut().for_each(|cell| *cell += 1));
        acc + (0..SIZE)
            .flat_map(|y| (0..SIZE).map(move |x| (x, y)))
            .fold(0, |acc, (x, y)| {
                acc + (parsed_input[y][x] > 9)
                    .then(|| flash(&mut parsed_input, x, y))
                    .unwrap_or(0)
            })
    })
}

pub fn p2(input: &str) -> usize {
    let mut parsed_input = parse_input(input);

    (1..)
        .find(|_| {
            parsed_input
                .iter_mut()
                .for_each(|row| row.iter_mut().for_each(|cell| *cell += 1));
            (0..SIZE)
                .flat_map(|y| (0..SIZE).map(move |x| (x, y)))
                .fold(0, |acc, (x, y)| {
                    acc + (parsed_input[y][x] > 9)
                        .then(|| flash(&mut parsed_input, x, y))
                        .unwrap_or(0)
                })
                == SIZE * SIZE
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#;
    const PROBLEM_INPUT: &str = include_str!("inputs/d11.txt");

    const EXAMPLE_P1_ANSWER: usize = 1656;
    const EXAMPLE_P2_ANSWER: usize = 195;
    const PROBLEM_P1_ANSWER: usize = 1785;
    const PROBLEM_P2_ANSWER: usize = 354;

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
