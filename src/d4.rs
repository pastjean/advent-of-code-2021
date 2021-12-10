use std::collections::HashSet;

const WINNING_PATTERNS: [(usize, usize, usize, usize, usize); 10] = [
    // Rows
    (0, 1, 2, 3, 4),
    (5, 6, 7, 8, 9),
    (10, 11, 12, 13, 14),
    (15, 16, 17, 18, 19),
    (20, 21, 22, 23, 24),
    // Column patterns
    (0, 5, 10, 15, 20),
    (1, 6, 11, 16, 21),
    (2, 7, 12, 17, 22),
    (3, 8, 13, 18, 23),
    (4, 9, 14, 19, 24),
];

#[derive(Clone, Debug)]
pub struct BingoCard {
    pub nums: Vec<usize>,
    pub marked_nums_idx: HashSet<usize>,
}

impl BingoCard {
    pub fn non_marked_nums(&self) -> Vec<usize> {
        self.nums
            .iter()
            .enumerate()
            .filter(|(i, _)| None == self.marked_nums_idx.get(i))
            .map(|(_, num)| *num)
            .collect()
    }
}

pub fn parse_input(input: &str) -> (Vec<usize>, Vec<BingoCard>) {
    // Parse first line
    let mut parts = input.trim().split("\n\n");

    let numbers: Vec<usize> = parts
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let cards = parts
        .map(|card| {
            let non_marked_nums = card
                .split_ascii_whitespace()
                .map(&str::parse::<usize>)
                .map(Result::unwrap)
                .collect::<Vec<_>>();

            BingoCard {
                nums: non_marked_nums,
                marked_nums_idx: HashSet::new(),
            }
        })
        .collect::<Vec<_>>();

    (numbers, cards)
}

pub fn day4_p1(input: &str) -> usize {
    let (numbers, mut cards) = parse_input(input);

    // play the game
    for number in numbers {
        for card in &mut cards {
            for (i, num) in card.nums.iter().enumerate() {
                if *num == number {
                    card.marked_nums_idx.insert(i);
                }
            }

            for pat in WINNING_PATTERNS {
                if let (Some(_), Some(_), Some(_), Some(_), Some(_)) = (
                    card.marked_nums_idx.get(&pat.0),
                    card.marked_nums_idx.get(&pat.1),
                    card.marked_nums_idx.get(&pat.2),
                    card.marked_nums_idx.get(&pat.3),
                    card.marked_nums_idx.get(&pat.4),
                ) {
                    let non_marked_sum: usize = card.non_marked_nums().iter().sum();
                    return number * non_marked_sum;
                }
            }
            // validate if you win
        }
    }
    0
}

pub fn day4_p2(input: &str) -> usize {
    let (numbers, mut cards) = parse_input(input);
    let mut cards_that_won: HashSet<usize> = HashSet::new();
    let mut winnings: Vec<usize> = Vec::new();

    // play the game
    for number in numbers {
        for (card_idx, card) in &mut cards.iter_mut().enumerate() {
            for (i, num) in card.nums.iter().enumerate() {
                if *num == number {
                    card.marked_nums_idx.insert(i);
                }
            }

            // this card won a long time ago skip;
            if cards_that_won.get(&card_idx).is_some() {
                continue;
            }

            for pat in WINNING_PATTERNS {
                if let (Some(_), Some(_), Some(_), Some(_), Some(_)) = (
                    card.marked_nums_idx.get(&pat.0),
                    card.marked_nums_idx.get(&pat.1),
                    card.marked_nums_idx.get(&pat.2),
                    card.marked_nums_idx.get(&pat.3),
                    card.marked_nums_idx.get(&pat.4),
                ) {
                    let non_marked_sum: usize = card.non_marked_nums().iter().sum();
                    winnings.push(number * non_marked_sum);
                    cards_that_won.insert(card_idx);
                }
            }
            // validate if you win
        }
    }

    *winnings.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day4_p1_example() {
        let input = include_str!("inputs/d4_example.txt");
        assert_eq!(day4_p1(input), 4512); // 188 * 24 = 4512
    }

    #[test]
    fn test_day4_p1() {
        let input = include_str!("inputs/d4.txt");
        assert_eq!(day4_p1(input), 72770); // 188 * 24 = 4512
    }

    #[test]
    fn test_day4_p2_example() {
        let input = include_str!("inputs/d4_example.txt");
        assert_eq!(day4_p2(input), 1924); // 188 * 24 = 4512
    }

    #[test]
    fn test_day4_p2() {
        let input = include_str!("inputs/d4.txt");
        assert_eq!(day4_p2(input), 13912); // 188 * 24 = 4512
    }
}
