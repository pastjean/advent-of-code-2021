use std::collections::HashSet;

fn parse_input(input: &str) -> (HashSet<(usize, usize)>, Vec<(u8, usize)>) {
    let (coords, folds) = input.trim().split_once("\n\n").unwrap();

    let coords: HashSet<(usize, usize)> = coords
        .lines()
        .map(|l| {
            let coord = l.split_once(',').unwrap();
            (
                coord.0.parse::<usize>().unwrap(),
                coord.1.parse::<usize>().unwrap(),
            )
        })
        .collect();

    let folds = folds
        .lines()
        .map(|l| l.trim_start_matches("fold along ").split_once('=').unwrap())
        .map(|(c, i)| (c.as_bytes()[0], i.parse::<usize>().unwrap()))
        .collect::<Vec<_>>();

    (coords, folds)
}

fn fold(hash_set: &mut HashSet<(usize, usize)>, fold: &(u8, usize)) {
    let points = hash_set.iter().copied().collect::<Vec<(usize, usize)>>();
    for point in points {
        match fold.0 {
            b'y' => {
                if point.1 > fold.1 {
                    hash_set.remove(&point);
                    hash_set.insert((point.0, fold.1 - (point.1 - fold.1)));
                }
            }
            _ => {
                if point.0 > fold.1 {
                    hash_set.remove(&point);
                    hash_set.insert((fold.1 - (point.0 - fold.1), point.1));
                }
            }
        }
    }
}

pub fn p1(input: &str) -> usize {
    let (mut coords, folds) = parse_input(input);
    fold(&mut coords, &folds[0]);

    coords.len()
}

pub fn p2(input: &str) -> String {
    let (mut coords, folds) = parse_input(input);
    for cur_fold in folds {
        fold(&mut coords, &cur_fold);
    }

    let max_x = coords
        .iter()
        .max_by(|point1, point2| point1.0.cmp(&point2.0))
        .unwrap()
        .0;
    let max_y = coords
        .iter()
        .max_by(|point1, point2| point1.1.cmp(&point2.1))
        .unwrap()
        .1;

    let mut str: String = String::new();
    for y in 0..=max_y {
        for x in 0..=max_x {
            if coords.contains(&(x, y)) {
                str.push('#');
            } else {
                str.push(' ');
            }
        }
        str.push('\n');
    }

    str
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("inputs/d13_example.txt");
    const PROBLEM_INPUT: &str = include_str!("inputs/d13.txt");

    const EXAMPLE_P1_ANSWER: usize = 17;
    const EXAMPLE_P2_ANSWER: &str = "#####\n#   #\n#   #\n#   #\n#####\n";
    const PROBLEM_P1_ANSWER: usize = 689;
    const PROBLEM_P2_ANSWER: &str = "###  #    ###   ##    ##  ##  #    #  #\n#  # #    #  # #  #    # #  # #    #  #\n#  # #    ###  #       # #    #    #  #\n###  #    #  # #       # # ## #    #  #\n# #  #    #  # #  # #  # #  # #    #  #\n#  # #### ###   ##   ##   ### ####  ## \n";

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
        let answer = p2(PROBLEM_INPUT);
        println!("{}", answer);
        assert_eq!(p2(PROBLEM_INPUT), PROBLEM_P2_ANSWER);
    }
}
