const INPUT: &str = include_str!("day01.txt");

fn main() {
    println!("Day NN Part 1: {:?}", part1(INPUT));
    println!("Day NN Part 2: {:?}", part2(INPUT));
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|s| s.trim())
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

fn count_increases(input: &[i32]) -> i32 {
    let mut input_iter = input.iter();
    let mut prev = input_iter.next();
    let mut next = input_iter.next();

    let mut count = 0;

    while let (Some(a), Some(b)) = (prev, next) {
        if a < b {
            count += 1;
        }

        prev = next;
        next = input_iter.next();
    }

    count
}

// replace return type as required by the problem
fn part1(input: &str) -> i32 {
    count_increases(&parse_input(input))
}

fn sum_three_element_windows(input: &[i32]) -> Vec<i32> {
    let mut iter = input.iter();
    let mut one = iter.next();
    let mut two = iter.next();
    let mut three = iter.next();
    let mut ret = vec![];

    while let (Some(a), Some(b), Some(c)) = (one, two, three) {
        ret.push(a + b + c);
        one = two;
        two = three;
        three = iter.next();
    }

    ret
}

// replace return type as required by the problem
fn part2(input: &str) -> i32 {
    count_increases(&sum_three_element_windows(&parse_input(input)))
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = include_str!("day01_test.txt");
    use super::*;
    use test_support::test_support::TestCase;

    #[test]
    fn test_part1() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 7,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part1(*input), *expected);
        }
    }

    #[test]
    fn test_part2() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 5,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part2(*input), *expected);
        }
    }
}
