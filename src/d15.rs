use std::collections::HashSet;
use std::mem;

pub struct Grid {
    width: usize,
    height: usize,
    data: Box<[usize]>,
}

impl Grid {
    pub fn parse_input(text: &[u8]) -> Self {
        let height = text.split(|&c| c == b'\n').count() - 1; // off by one, idk
        let width = text.split(|&c| c == b'\n').next().unwrap().len();
        let data = text
            .iter()
            .filter(|&&c| c != b'\n')
            .map(|c| usize::from(c - b'0'))
            .collect();
        Self {
            height,
            width,
            data,
        }
    }

    pub fn bfs(&self) -> usize {
        let mut flood = Self {
            width: self.width,
            height: self.height,
            data: self.data.iter().map(|_| usize::MAX).collect(),
        };
        flood.data[0] = 0;
        let mut scan = HashSet::new();
        let mut scan_next = HashSet::new();
        scan.insert((0, 0));
        loop {
            if scan.is_empty() {
                return flood.get(self.width - 1, self.height - 1).unwrap();
            }
            for (x, y) in scan.drain() {
                let v = flood.get(x, y).unwrap();
                let mut f = |x, y| {
                    let v = v + self.get(x, y).unwrap();
                    if v < flood.get(x, y).unwrap() {
                        flood.set(x, y, v).unwrap();
                        scan_next.insert((x, y));
                    }
                };
                (x < self.width - 1).then(|| f(x + 1, y));
                (y < self.height - 1).then(|| f(x, y + 1));
                (x > 0).then(|| f(x - 1, y));
                (y > 0).then(|| f(x, y - 1));
            }
            mem::swap(&mut scan, &mut scan_next);
        }
    }

    pub fn enlarge(self, n: usize) -> Self {
        let (sx, sy) = (self.width, self.height);
        let f = |x, y, mx, my| -> usize { (mx + my + self.get(x, y).unwrap() - 1) % 9 + 1 };
        Self {
            width: self.width * n,
            height: self.height * n,
            data: (0..n)
                .flat_map(move |my| {
                    (0..sy).flat_map(move |y| {
                        (0..n).flat_map(move |mx| (0..sx).map(move |x| f(x, y, mx, my)))
                    })
                })
                .collect(),
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<usize> {
        (x < self.width && y < self.height).then(|| self.data[y * self.width + x])
    }

    fn set(&mut self, x: usize, y: usize, value: usize) -> Option<()> {
        (x < self.width && y < self.height).then(|| self.data[y * self.width + x] = value)
    }
}

pub fn p1(input: &str) -> usize {
    let grid = Grid::parse_input(input.as_bytes());
    grid.bfs()
}

pub fn p2(input: &str) -> usize {
    let grid = Grid::parse_input(input.as_bytes());
    grid.enlarge(5).bfs()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
"#;
    const PROBLEM_INPUT: &str = include_str!("inputs/d15.txt");

    const EXAMPLE_P1_ANSWER: usize = 40;
    const EXAMPLE_P2_ANSWER: usize = 315;
    const PROBLEM_P1_ANSWER: usize = 790;
    const PROBLEM_P2_ANSWER: usize = 2998;

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
