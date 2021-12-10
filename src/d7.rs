pub fn parse_input(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(',')
        .map(|c| c.parse::<i32>().unwrap())
        .collect()
}

pub fn p1(input: &str) -> i32 {
    let mut crab_positions = parse_input(input);
    crab_positions.sort_unstable();

    let median = crab_positions[crab_positions.len() / 2];

    crab_positions.iter().map(|pos| (pos - median).abs()).sum()
}

pub fn p2(input: &str) -> i32 {
    let crab_positions = parse_input(input);
    let mean_f: f32 = crab_positions.iter().sum::<i32>() as f32 / crab_positions.len() as f32;

    let ceil = mean_f.ceil() as i32;

    let ceil_sum: i32 = crab_positions
        .iter()
        .map(|pos| {
            let travel_distance = (pos - ceil).abs();
            (travel_distance * (travel_distance + 1)) / 2
        })
        .sum();

    ceil_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_p1_example() {
        let input = EXAMPLE;
        assert_eq!(p1(input), 37);
    }

    #[test]
    fn test_p1() {
        let input = include_str!("inputs/d7.txt");
        assert_eq!(p1(input), 355521);
    }

    #[test]
    fn test_p2_example() {
        let input = EXAMPLE;
        assert_eq!(p2(input), 168);
    }

    #[test]
    fn test_p2() {
        let input = include_str!("inputs/d7.txt");
        p2(EXAMPLE);
        assert_eq!(p2(input), 100148861);
    }
}
