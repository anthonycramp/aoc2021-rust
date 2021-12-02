const INPUT: &str = include_str!("day02.txt");
const TEST_INPUT: &str = include_str!("day02_test.txt");

fn main() {
    println!("Day 02 Part 1: {:?}", part1(INPUT));
    println!("Day 02 Part 2: {:?}", part2(INPUT));
}

// replace return type as required by the problem
fn part1(input: &str) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;

    for line in input.lines() {
        let mut command = line.split(" ");
        let operation = command.next();
        let distance = command.next().unwrap().parse::<i32>().unwrap();

        match operation {
            Some("forward") => horizontal += distance,
            Some("down") => depth += distance,
            Some("up") => depth -= distance,
            Some(c) => println!("Uknown command {}", c),
            _ => panic!("dang"),
        }
    }

    horizontal * depth
}

// replace return type as required by the problem
fn part2(input: &str) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in input.lines() {
        let mut command = line.split(" ");
        let operation = command.next();
        let distance = command.next().unwrap().parse::<i32>().unwrap();

        match operation {
            Some("forward") => {
                horizontal += distance;
                depth += aim * distance
            }
            Some("down") => aim += distance,
            Some("up") => aim -= distance,
            Some(c) => println!("Uknown command {}", c),
            _ => panic!("dang"),
        }
    }

    horizontal * depth
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_support::test_support::TestCase;

    #[test]
    fn test_part1() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 150,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part1(*input), *expected);
        }
    }

    #[test]
    fn test_part2() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 900,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part2(*input), *expected);
        }
    }
}
