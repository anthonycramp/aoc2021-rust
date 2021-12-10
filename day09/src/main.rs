const INPUT: &str = include_str!("day09.txt");

fn main() {
    println!("Day 09 Part 1: {:?}", part1(INPUT));
    println!("Day 09 Part 2: {:?}", part2(INPUT));
}

struct HeightMap {
    width: usize,
    map: Vec<u32>,
}

impl From<&str> for HeightMap {
    fn from(input: &str) -> Self {
        let mut height_map = HeightMap {
            width: 0,
            map: vec![],
        };
        for line in input.lines() {
            height_map.width = line.len();
            for c in line.chars() {
                height_map.map.push(c.to_digit(10).unwrap());
            }
        }

        height_map
    }
}

impl HeightMap {
    fn get_neighbours(&self, location: usize) -> Vec<u32> {
        // neighbours are at -1, +1, -width, and +width
        let mut neighbours = vec![];

        if location % self.width != 0 {
            // not at left edge
            if let Some(left) = location.checked_sub(1) {
                neighbours.push(self.map[left]);
            }
        }

        if let Some(up) = location.checked_sub(self.width) {
            neighbours.push(self.map[up]);
        }

        if (location + 1) % self.width != 0 {
            // not at right edge
            if location + 1 < self.map.len() {
                neighbours.push(self.map[location + 1]);
            }
        }

        if location + self.width < self.map.len() {
            neighbours.push(self.map[location + self.width]);
        }

        neighbours
    }

    fn find_low_points(&self) -> Vec<(usize, u32)> {
        let mut low_points = vec![];

        for (i, v) in self.map.iter().enumerate() {
            let neighbours = self.get_neighbours(i);
            if !neighbours.iter().any(|&v2| v2 <= *v) {
                low_points.push((i, *v));
            }
        }

        low_points
    }
}

// replace return type as required by the problem
fn part1(input: &str) -> u32 {
    let height_map = HeightMap::from(input);
    let low_points = height_map.find_low_points();
    low_points.iter().map(|&(_, v)| v + 1).sum::<u32>()
}

// replace return type as required by the problem
fn part2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = include_str!("day09_test.txt");
    use super::*;
    use test_support::test_support::TestCase;

    #[test]
    fn test_part1() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 15,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part1(*input), *expected);
        }
    }

    #[test]
    fn test_part2() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 1134,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part2(*input), *expected);
        }
    }
}
