const INPUT: &str = include_str!("day03.txt");

fn main() {
    println!("Day 03 Part 1: {:?}", part1(INPUT));
    println!("Day 03 Part 2: {:?}", part2(INPUT));
}

// replace return type as required by the problem
fn part1(input: &str) -> i32 {
    let lines = input.lines();
    let mut number_of_records = 0;
    let mut gamma_rate = String::from("");
    let mut epsilon_rate = String::from("");

    let mut one_counts = vec![];

    for line in lines {
        for (i, c) in line.chars().enumerate() {
            if one_counts.len() <= i {
                one_counts.push(0);
            }

            if c == '1' {
                one_counts[i] += 1;
            }
        }
        number_of_records += 1;
    }

    for count in one_counts {
        if count <= number_of_records / 2 {
            epsilon_rate += "1";
            gamma_rate += "0";
        } else {
            epsilon_rate += "0";
            gamma_rate += "1";
        }
    }

    let gamma_rate = i32::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon_rate = i32::from_str_radix(&epsilon_rate, 2).unwrap();
    gamma_rate * epsilon_rate
}

// replace return type as required by the problem
fn part2(input: &str) -> i32 {
    let mut lines: Vec<String> = input.lines().map(String::from).collect();

    let mut binary_index = 0;

    let oxygen_generator_rating = loop {
        if lines.len() == 1 {
            break lines[0].clone();
        }
        let mut number_of_records = 0;
        let mut number_of_ones = 0;
        for line in &lines {
            if line.chars().nth(binary_index).unwrap() == '1' {
                number_of_ones += 1;
            }
            number_of_records += 1;
        }

        let number_of_zeros = number_of_records - number_of_ones;
        let mut filter_char = '0';
        if number_of_ones >= number_of_zeros {
            filter_char = '1';
        }
        lines = lines
            .iter()
            .filter(|l| l.chars().nth(binary_index).unwrap() == filter_char)
            .cloned()
            .collect();

        binary_index += 1;
    };

    binary_index = 0;
    lines = input.lines().map(String::from).collect();
    let co2_scrubber_rating = loop {
        if lines.len() == 1 {
            break lines[0].clone();
        }
        let mut number_of_records = 0;
        let mut number_of_ones = 0;
        for line in &lines {
            if line.chars().nth(binary_index).unwrap() == '1' {
                number_of_ones += 1;
            }
            number_of_records += 1;
        }

        let number_of_zeros = number_of_records - number_of_ones;
        let mut filter_char = '1';
        if number_of_ones >= number_of_zeros {
            filter_char = '0';
        }
        lines = lines
            .iter()
            .filter(|l| l.chars().nth(binary_index).unwrap() == filter_char)
            .cloned()
            .collect();

        binary_index += 1;
    };

    let oxygen_generator_rating = i32::from_str_radix(&oxygen_generator_rating, 2).unwrap();
    let co2_scubber_rating = i32::from_str_radix(&co2_scrubber_rating, 2).unwrap();

    oxygen_generator_rating * co2_scubber_rating
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = include_str!("day03_test.txt");
    use super::*;
    use test_support::test_support::TestCase;

    #[test]
    fn test_part1() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 198,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part1(*input), *expected);
        }
    }

    #[test]
    fn test_part2() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 230,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part2(*input), *expected);
        }
    }
}
