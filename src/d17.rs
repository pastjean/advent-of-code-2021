use std::ops::RangeInclusive;

pub fn p1(input: (RangeInclusive<i32>, RangeInclusive<i32>)) -> i32 {
    let maximum_y_drop = input.1.start();
    maximum_y_drop * (maximum_y_drop + 1) / 2
}

pub fn p2(input: (RangeInclusive<i32>, RangeInclusive<i32>)) -> i32 {
    let input = BoundingBox {
        x: input.0,
        y: input.1,
    };

    let min_x = *input.x.start();
    let max_x = *input.x.end();
    let min_y = *input.y.start();
    let max_y = *input.y.end();
    // compute upper bound of steps from maximum y drop
    // the largest step count gives the tallest arc
    //
    // collect the intervals of "accuracy" for each step count
    let intervals: Vec<_> = (1..=(-min_y + 1) * 2)
        .map(|steps| {
            // solve bounds for yv
            // n / 2 * (yv - n + 1) is in (min_y, max_y)
            // (slightly easier since yv can be negative)

            // 2 / n * m_y + n - 1 ><= yv
            let yv_min =
                ((2 * min_y + steps * steps - steps) as f64 / 2.0 / steps as f64).ceil() as i32;
            let yv_max =
                ((2 * max_y + steps * steps - steps) as f64 / 2.0 / steps as f64).floor() as i32;

            // solve bounds for xv
            // simple case: n / 2 * (xv - n + 1) is in (min_x, max_x)

            // 2 / n * m_x + n - 1 ><= xv
            let xv_min =
                ((2 * min_x + steps * steps - steps) as f64 / 2.0 / steps as f64).ceil() as i32;
            let maybe_xv_max =
                ((2 * max_x + steps * steps - steps) as f64 / 2.0 / steps as f64).floor() as i32;

            // Special case when xv falls to zero and stays there (since it can't fall to negatives)
            // (xv * xv + xv) / 2 in minmax
            // Solve quadratic
            let special_xv_min = (-1 + (1.0 + 8.0 * min_x as f64).sqrt().ceil() as i32) / 2;
            let special_xv_max = (-1 + (1.0 + 8.0 * max_x as f64).sqrt().floor() as i32) / 2;

            // exclude velocities that overshoot and "curve back left"
            let xv_max = maybe_xv_max.min(2 * max_x / steps);

            let x_range = if xv_min <= xv_max || special_xv_max > steps {
                xv_min..=xv_max
            } else {
                special_xv_min..=special_xv_max
            };
            let y_range = yv_min..=yv_max;
            // The rectangular interval
            BoundingBox {
                x: x_range,
                y: y_range,
            }
        })
        .collect();

    // compute the area of unique regions in the rectangles
    // the data guarantees that every rectangle overlap is between consecutive elements
    // (i.e. if you are in the target for N steps using a given velocity, it's always N consecutive steps)
    // furthermore, if more than two rectangles {R1, R2, R3, ...} intersect, the intersection R1 & R2
    // is a superset of the intersection R1 & (R3 | R4 | ...)
    // (this follows from the first guarantee)
    intervals[0].area()
        + intervals
            .windows(2)
            .map(|window| window[1].area() - window[1].intersection(&window[0]).area())
            .sum::<i32>()
}

pub struct BoundingBox {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
}

impl BoundingBox {
    pub fn width(&self) -> i32 {
        0.max(self.x.end() - self.x.start() + 1)
    }

    pub fn height(&self) -> i32 {
        0.max(self.y.end() - self.y.start() + 1)
    }

    pub fn area(&self) -> i32 {
        self.width() * self.height()
    }

    pub fn intersection(&self, other: &Self) -> BoundingBox {
        BoundingBox {
            x: *self.x.start().max(other.x.start())..=*self.x.end().min(other.x.end()),
            y: *self.y.start().max(other.y.start())..=*self.y.end().min(other.y.end()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROBLEM_INPUT: (RangeInclusive<i32>, RangeInclusive<i32>) = (269..=292, -68..=-44);
    const PROBLEM_P1_ANSWER: i32 = 0;
    const PROBLEM_P2_ANSWER: i32 = 0;

    #[test]
    fn test_problem_p1() {
        assert_eq!(p1(PROBLEM_INPUT), PROBLEM_P1_ANSWER);
    }

    #[test]
    fn test_problem_p2() {
        assert_eq!(p2(PROBLEM_INPUT), PROBLEM_P2_ANSWER);
    }
}
