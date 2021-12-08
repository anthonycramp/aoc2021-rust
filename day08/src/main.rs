const INPUT: &str = include_str!("day08.txt");

fn main() {
    println!("Day 08 Part 1: {:?}", part1(INPUT));
    println!("Day 08 Part 2: {:?}", part2(INPUT));
}

// replace return type as required by the problem
fn part1(input: &str) -> i32 {
    let lines = input.lines();
    let mut counts: Vec<i32> = vec![0; 10];

    for line in lines {
        let mut input_output = line.split(" | ");
        let output = input_output.nth(1).unwrap();
        let output_seven_segments = output.split_ascii_whitespace();
        for output_seven_segment in output_seven_segments {
            match output_seven_segment.len() {
                2 => counts[1] += 1,
                3 => counts[7] += 1,
                4 => counts[4] += 1,
                7 => counts[8] += 1,
                _ => (),
            }
        }
    }

    counts.iter().sum::<i32>() as i32
}

// replace return type as required by the problem
fn part2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = include_str!("day08_test.txt");
    use super::*;
    use test_support::test_support::TestCase;

    #[test]
    fn test_part1() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 26,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part1(*input), *expected);
        }
    }

    #[test]
    fn test_part2() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 61229,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part2(*input), *expected);
        }
    }
}
