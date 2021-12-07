const INPUT: &str = include_str!("day06.txt");

fn main() {
    println!("Day 06 Part 1: {:?}", part1(INPUT));
    println!("Day 06 Part 2: {:?}", part2(INPUT));
}

fn parse_initial_lantern_fish(input: &str) -> Vec<usize> {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect()
}

fn create_lantern_fish(lantern_fish_lives: &[usize], days: usize) -> i64 {
    let mut new_lantern_fish_per_day = vec![0; days];

    let number_of_weeks: usize = days / 7;
    for lantern_fish_life in lantern_fish_lives {
        for i in 0..=number_of_weeks {
            let next_gen = lantern_fish_life + i * 7;
            if next_gen < days {
                new_lantern_fish_per_day[next_gen] += 1;
            }
        }
    }

    for i in 0..days {
        let mut next_gen = i + 9;
        while next_gen < days {
            new_lantern_fish_per_day[next_gen] += new_lantern_fish_per_day[i];
            next_gen += 7;
        }
    }
    let new_lantern_fish: i64 = new_lantern_fish_per_day.iter().sum();
    lantern_fish_lives.len() as i64 + new_lantern_fish
}

// replace return type as required by the problem
fn part1(input: &str) -> i64 {
    create_lantern_fish(&parse_initial_lantern_fish(input), 80)
}

// replace return type as required by the problem
fn part2(input: &str) -> i64 {
    create_lantern_fish(&parse_initial_lantern_fish(input), 256)
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = include_str!("day06_test.txt");
    use super::*;
    use test_support::test_support::TestCase;

    #[test]
    fn test_part1() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 5934,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part1(*input), *expected);
        }
    }

    #[test]
    fn test_part2() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 26984457539,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part2(*input), *expected);
        }
    }
}
