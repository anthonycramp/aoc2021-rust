const INPUT: &str = include_str!("day10.txt");

fn main() {
    println!("Day 10 Part 1: {:?}", part1(INPUT));
    println!("Day 10 Part 2: {:?}", part2(INPUT));
}

// replace return type as required by the problem
fn part1(input: &str) -> i32 {
    0
}

// replace return type as required by the problem
fn part2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = include_str!("day10_test.txt");
    use super::*;
    use test_support::test_support::TestCase;

    #[test]
    fn test_part1() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 26397,
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
