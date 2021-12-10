use std::collections::HashMap;

pub type Point = (i32, i32); // x,y
pub type Line = (Point, Point); // start,end

pub fn parse_point(input_point: &str) -> Point {
    let coords = input_point
        .trim()
        .split(',')
        .map(|input_coord| input_coord.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    (coords[0], coords[1])
}
pub fn parse_line(input_line: &str) -> Line {
    let points = input_line.split("->").map(parse_point).collect::<Vec<_>>();

    (points[0], points[1])
}

pub fn parse_input(input: &str) -> Vec<Line> {
    input.lines().map(parse_line).collect()
}

pub fn expand_line(line: &Line, diag: bool) -> Vec<Point> {
    let ((x1, y1), (x2, y2)) = *line;

    // diagonal line
    if x1 != x2 && y1 != y2 {
        if !diag {
            return vec![];
        }

        let ((x1, y1), (x2, y2)) = *line;
        let slope = (y2 - y1) / (x2 - x1);
        let intercept = y2 - (x2 * slope);

        let points = (x1.min(x2)..=x1.max(x2))
            .into_iter()
            .map(|x| (x, slope * x + intercept))
            .collect();
        return points;
    }

    let mut v = Vec::new();
    for x in x1.min(x2)..=x1.max(x2) {
        for y in y1.min(y2)..=y1.max(y2) {
            v.push((x, y))
        }
    }
    v
}

pub fn d5(input: &str, diagonals: bool) -> i32 {
    parse_input(input)
        .iter()
        .map(|l| expand_line(l, diagonals))
        .flatten()
        .fold(HashMap::new(), |mut points, coord| {
            let p = points.entry(coord).or_insert(0);
            *p += 1;
            points
        })
        .iter()
        .fold(0, |point, entry| {
            if *entry.1 > 1 {
                return point + 1;
            }
            point
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day5_p1_example() {
        let input = include_str!("inputs/d5_example.txt");
        assert_eq!(d5(input, false), 5);
    }

    #[test]
    fn test_day5_p1() {
        let input = include_str!("inputs/d5.txt");
        assert_eq!(d5(input, false), 5585); // 188 * 24 = 4512
    }

    #[test]
    fn test_day5_p2_example() {
        let input = include_str!("inputs/d5_example.txt");
        assert_eq!(d5(input, true), 12);
    }

    #[test]
    fn test_day5_p2() {
        let input = include_str!("inputs/d5.txt");
        assert_eq!(d5(input, true), 17193); // 188 * 24 = 4512
    }
}
