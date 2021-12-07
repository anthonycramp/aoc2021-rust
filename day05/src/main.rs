const INPUT: &str = include_str!("day05.txt");

fn main() {
    println!("Day NN Part 1: {:?}", part1(INPUT));
    println!("Day NN Part 2: {:?}", part2(INPUT));
}

enum LineSegmentOrientation {
    Horizontal,
    Vertical,
    Diagonal,
}

struct OrthogonalLineSegment {
    min: usize,
    max: usize,
    fixed: usize,
    orientation: LineSegmentOrientation,
}

impl OrthogonalLineSegment {
    fn new(p1: usize, p2: usize, fixed: usize, orientation: LineSegmentOrientation) -> Self {
        if p1 < p2 {
            Self {
                min: p1,
                max: p2,
                fixed,
                orientation,
            }
        } else {
            Self {
                min: p2,
                max: p1,
                fixed,
                orientation,
            }
        }
    }
}

// replace return type as required by the problem
fn part1(input: &str) -> i32 {
    let mut line_segments = vec![];

    for line in input.lines() {
        let mut splits = line.split_ascii_whitespace();
        let p1 = splits.next().unwrap();
        let _ = splits.next();
        let p2 = splits.next().unwrap();

        let mut p1 = p1.split(',');
        let x1 = p1.next().unwrap().parse().unwrap();
        let y1 = p1.next().unwrap().parse().unwrap();

        let mut p2 = p2.split(',');
        let x2 = p2.next().unwrap().parse().unwrap();
        let y2 = p2.next().unwrap().parse().unwrap();

        if x1 == x2 {
            line_segments.push(OrthogonalLineSegment::new(
                y1,
                y2,
                x1,
                LineSegmentOrientation::Vertical,
            ));
        } else if y1 == y2 {
            line_segments.push(OrthogonalLineSegment::new(
                x1,
                x2,
                y1,
                LineSegmentOrientation::Horizontal,
            ));
        }
    }

    let mut min_x = 0;
    let mut max_x = 1000;
    let mut min_y = 0;
    let mut max_y = 1000;

    dbg!(min_y, max_y, min_x, max_x);
    let rows = max_y - min_y + 1;
    let cols = max_x - min_x + 1;
    dbg!(rows, cols);
    let mut sea_floor = vec![0; rows * cols];

    for line_segment in &line_segments {
        for i in line_segment.min..=line_segment.max {
            match line_segment.orientation {
                LineSegmentOrientation::Horizontal => {
                    let pixel = line_segment.fixed * cols + i;
                    // dbg!(pixel);
                    sea_floor[pixel] += 1;
                }
                LineSegmentOrientation::Vertical => {
                    sea_floor[i * cols + line_segment.fixed] += 1;
                }
                _ => (),
            }
        }
    }

    sea_floor.iter().filter(|&n| *n >= 2).count() as i32
}

// replace return type as required by the problem
fn part2(input: &str) -> i32 {
    0
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
            expected: 123,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part2(*input), *expected);
        }
    }
}
