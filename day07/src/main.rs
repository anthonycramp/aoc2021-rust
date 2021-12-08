const INPUT: &str = include_str!("day07.txt");

fn main() {
    println!("Day 07 Part 1: {:?}", part1(INPUT));
    println!("Day 07 Part 2: {:?}", part2(INPUT));
}

fn get_fuel_to_align_at_position(crab_positions: &[i32], target_position: i32) -> i32 {
    let mut fuel_used = 0;
    for crab_position in crab_positions {
        fuel_used += (crab_position - target_position).abs();
    }
    fuel_used
}

fn get_fuel_to_align_at_position2(crab_positions: &[i32], target_position: i32) -> i32 {
    let mut fuel_used = 0;
    for crab_position in crab_positions {
        let distance_from_target = (crab_position - target_position).abs();
        fuel_used += distance_from_target * (distance_from_target + 1) / 2;
    }
    fuel_used
}

// replace return type as required by the problem
fn part1(input: &str) -> i32 {
    let crab_positions: Vec<i32> = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    let mut min_fuel = i32::MAX;
    for i in 0..crab_positions.len() {
        let fuel_used = get_fuel_to_align_at_position(&crab_positions, i as i32);
        if fuel_used < min_fuel {
            min_fuel = fuel_used;
        }
    }
    min_fuel
}

// replace return type as required by the problem
fn part2(input: &str) -> i32 {
    let crab_positions: Vec<i32> = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    let mut min_fuel = i32::MAX;
    for i in 0..crab_positions.len() {
        let fuel_used = get_fuel_to_align_at_position2(&crab_positions, i as i32);
        if fuel_used < min_fuel {
            min_fuel = fuel_used;
        }
    }
    min_fuel
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = include_str!("day07_test.txt");
    use super::*;
    use test_support::test_support::TestCase;

    #[test]
    fn test_part1() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 37,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part1(*input), *expected);
        }
    }

    #[test]
    fn test_part2() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 168,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part2(*input), *expected);
        }
    }
}
