const INPUT: &str = include_str!("day05.txt");

fn main() {
    println!("Day NN Part 1: {:?}", part1(INPUT));
    println!("Day NN Part 2: {:?}", part2(INPUT));
}

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl From<&str> for Point {
    fn from(input: &str) -> Self {
        let mut params = input.split(',');
        Point {
            x: params.next().unwrap().parse().unwrap(),
            y: params.next().unwrap().parse().unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
struct LineSegment {
    p1: Point,
    p2: Point,
}

impl From<&str> for LineSegment {
    fn from(input: &str) -> Self {
        let mut params = input.split_ascii_whitespace();

        LineSegment {
            p1: Point::from(params.next().unwrap()),
            p2: Point::from(params.nth(1).unwrap()),
        }
    }
}

impl LineSegment {
    fn draw(&self, sea_floor: &mut SeaFloor) {
        let delta_x: i32 = if self.p2.x < self.p1.x {
            -1
        } else if self.p2.x > self.p1.x {
            1
        } else {
            0
        };
        let delta_y: i32 = if self.p2.y < self.p1.y {
            -1
        } else if self.p2.y > self.p1.y {
            1
        } else {
            0
        };
        let mut p = self.p1.clone();
        while p != self.p2 {
            sea_floor.draw(&p);
            p.x += delta_x;
            p.y += delta_y;
        }
        sea_floor.draw(&self.p2);
    }

    fn is_horizontal(&self) -> bool {
        self.p1.y == self.p2.y
    }

    fn is_vertical(&self) -> bool {
        self.p1.x == self.p2.x
    }
}

struct SeaFloor {
    sea_floor: Vec<i32>,
    width: i32,
}

impl Default for SeaFloor {
    fn default() -> Self {
        let width = 1000;
        let height = 1000;
        Self {
            sea_floor: vec![0; (width * height).try_into().unwrap()],
            width,
        }
    }
}

impl SeaFloor {
    fn draw(&mut self, p: &Point) {
        let pixel: i32 = p.y * self.width + p.x;
        self.sea_floor[pixel as usize] += 1;
    }

    fn count(&self) -> i32 {
        self.sea_floor
            .iter()
            .filter(|&p| *p >= 2)
            .count()
            .try_into()
            .unwrap()
    }
}

// replace return type as required by the problem
fn part1(input: &str) -> i32 {
    let line_segments: Vec<LineSegment> = input.lines().map(LineSegment::from).collect();
    let mut sea_floor = SeaFloor::default();

    for line_segment in &line_segments {
        if line_segment.is_horizontal() || line_segment.is_vertical() {
            line_segment.draw(&mut sea_floor);
        }
    }
    sea_floor.count()
}

// replace return type as required by the problem
fn part2(input: &str) -> i32 {
    let line_segments: Vec<LineSegment> = input.lines().map(LineSegment::from).collect();
    let mut sea_floor = SeaFloor::default();

    for line_segment in &line_segments {
        line_segment.draw(&mut sea_floor);
    }
    sea_floor.count()
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = include_str!("day05_test.txt");
    use super::*;
    use test_support::test_support::TestCase;

    #[test]
    fn test_part1() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 5,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part1(*input), *expected);
        }
    }

    #[test]
    fn test_part2() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 12,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part2(*input), *expected);
        }
    }
}
