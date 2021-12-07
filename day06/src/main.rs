const INPUT: &str = include_str!("day06.txt");

fn main() {
    println!("Day 06 Part 1: {:?}", part1(INPUT));
    println!("Day 06 Part 2: {:?}", part2(INPUT));
}

// replace return type as required by the problem
fn part1(input: &str) -> i32 {
    let mut lantern_fish_lives: Vec<i32> = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    for _ in 0..80 {
        let mut new_lantern_fish = 0;
        for lantern_fish_life in &mut lantern_fish_lives {
            if *lantern_fish_life == 0 {
                *lantern_fish_life = 6;
                new_lantern_fish += 1;
            } else {
                *lantern_fish_life -= 1;
            }
        }

        for _ in 0..new_lantern_fish {
            lantern_fish_lives.push(8);
        }
    }
    lantern_fish_lives.len() as i32
}

// replace return type as required by the problem
fn part2(input: &str) -> i32 {
    0
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
            expected: 123,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part2(*input), *expected);
        }
    }
}
