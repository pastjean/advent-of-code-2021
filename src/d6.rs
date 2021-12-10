use std::collections::HashMap;

use itertools::Itertools;

pub fn parse_input(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|c| c.parse::<usize>().unwrap())
        .collect()
}

pub fn world(input: &str, ticks: usize) -> usize {
    let mut world = parse_input(input).into_iter().counts();

    for _tick in 0..ticks {
        let mut next_world: HashMap<usize, usize> = HashMap::new();

        for (state, count) in world {
            // Spawn new stuff
            if state == 0 {
                next_world.insert(8, count);
            }

            let next_state = match state {
                0 => 6,
                i => i - 1,
            };

            let entry = next_world.entry(next_state).or_insert(0);
            *entry += count;
        }

        world = next_world;
    }

    world.iter().map(|(_, &v)| v).sum()
}

pub fn p1(input: &str) -> usize {
    world(input, 80)
}

pub fn p2(input: &str) -> usize {
    world(input, 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_example() {
        let input = include_str!("inputs/d6_example.txt");
        assert_eq!(p1(input), 5934);
    }

    #[test]
    fn test_p1() {
        let input = include_str!("inputs/d6.txt");
        assert_eq!(p1(input), 386755);
    }

    #[test]
    fn test_p2_example() {
        let input = include_str!("inputs/d6_example.txt");
        assert_eq!(p2(input), 26984457539);
    }

    #[test]
    fn test_p2() {
        let input = include_str!("inputs/d6.txt");
        assert_eq!(p2(input), 1732731810807);
    }
}
