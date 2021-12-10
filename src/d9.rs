use std::collections::HashSet;

struct Map {
    values: Vec<u8>,
    width: i32,
    height: i32,
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let lines = input.trim().lines().enumerate();

        let (values, width, height) = lines.fold(
            (Vec::new(), 0, 0),
            |(mut points, _, _), (line_idx, line)| {
                let mut new_points = line
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect::<Vec<u8>>();
                points.append(&mut new_points);
                (points, line.len() as i32, (line_idx + 1) as i32)
            },
        );

        Map {
            values,
            width,
            height,
        }
    }
}

impl Map {
    fn get(&self, x: i32, y: i32) -> Option<u8> {
        let in_bounds = x >= 0 && x < self.width && y >= 0 && y < self.height;
        in_bounds.then(|| self.values[(x + y * self.width) as usize])
    }
}

fn get_low_points(grid: &Map) -> HashSet<(i32, i32)> {
    (0..grid.width)
        .flat_map(|x| {
            (0..grid.height).filter_map(move |y| {
                let center = grid.get(x, y).unwrap();
                let ids = [(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)];

                let basin = ids
                    .iter()
                    .filter_map(|(xp, yp)| grid.get(*xp, *yp))
                    .all(|x| center < x);

                if basin {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn flood_recurs<'a>(
    grid: &Map,
    x: i32,
    y: i32,
    pts: &'a mut HashSet<(i32, i32)>,
) -> &'a mut HashSet<(i32, i32)> {
    if pts.insert((x, y)) {
        for (xp, yp) in [(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)] {
            if let Some(val) = grid.get(xp, yp) {
                if val != 9 {
                    flood_recurs(grid, xp, yp, pts);
                }
            }
        }
    }

    pts
}

fn basin_size(grid: &Map, x: i32, y: i32) -> usize {
    let mut pts = HashSet::new();
    flood_recurs(grid, x, y, &mut pts).len()
}

pub fn p1(input: &str) -> u32 {
    let map = Map::from(input);

    let low_points = get_low_points(&map);
    low_points
        .iter()
        .map(|(x, y)| map.get(*x, *y).unwrap() as u32 + 1)
        .sum()
}

pub fn p2(input: &str) -> usize {
    let map = Map::from(input);

    let low_points = get_low_points(&map);
    let mut basin_sizes: Vec<usize> = low_points
        .iter()
        .map(|(x, y)| basin_size(&map, *x, *y))
        .collect();

    basin_sizes.sort_unstable();
    basin_sizes.iter().rev().take(3).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"2199943210
3987894921
9856789892
8767896789
9899965678"#;

    #[test]
    fn test_p1_example() {
        assert_eq!(p1(EXAMPLE), 15);
    }

    #[test]
    fn test_p1() {
        let input = include_str!("inputs/d9.txt");
        assert_eq!(p1(input), 591);
    }

    #[test]
    fn test_p2_example() {
        let input = EXAMPLE;
        assert_eq!(p2(input), 1134);
    }

    #[test]
    fn test_p2() {
        let input = include_str!("inputs/d9.txt");
        assert_eq!(p2(input), 1113424);
    }
}
