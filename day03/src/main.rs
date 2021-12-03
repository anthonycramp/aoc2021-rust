const INPUT: &str = include_str!("day03.txt");

fn main() {
    println!("Day 03 Part 1: {:?}", part1(INPUT));
    println!("Day 03 Part 2: {:?}", part2(INPUT));
}

// replace return type as required by the problem
fn part1(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let number_of_records = lines.len();
    let record_size = lines[0].len();

    let mut one_counts = vec![0; record_size];

    for line in lines {
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                one_counts[i] += 1;
            }
        }
    }

    let mut gamma_rate = String::from("");
    let mut epsilon_rate = String::from("");

    for one_count in one_counts {
        let zero_count = number_of_records - one_count;
        if one_count <= zero_count {
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

enum Device {
    OxygenGenerator,
    Co2Scrubber,
}

fn get_device_rating(diagnostics: &str, device: Device) -> i32 {
    let mut lines: Vec<String> = diagnostics.lines().map(String::from).collect();

    let mut binary_index = 0;

    let device_rating = loop {
        let number_of_reports = lines.len();
        if number_of_reports == 1 {
            break lines[0].clone();
        }

        let number_of_ones = lines
            .iter()
            .filter(|l| l.chars().nth(binary_index).unwrap() == '1')
            .count();
        let number_of_zeros = number_of_reports - number_of_ones;

        let filter_char = if number_of_ones >= number_of_zeros {
            match device {
                Device::OxygenGenerator => '1',
                Device::Co2Scrubber => '0',
            }
        } else {
            match device {
                Device::OxygenGenerator => '0',
                Device::Co2Scrubber => '1',
            }
        };

        lines = lines
            .iter()
            .filter(|l| l.chars().nth(binary_index).unwrap() == filter_char)
            .cloned()
            .collect();

        binary_index += 1;
    };

    i32::from_str_radix(&device_rating, 2).unwrap()
}

// replace return type as required by the problem
fn part2(input: &str) -> i32 {
    let oxygen_generator_rating = get_device_rating(input, Device::OxygenGenerator);
    let co2_scrubber_rating = get_device_rating(input, Device::Co2Scrubber);

    oxygen_generator_rating * co2_scrubber_rating
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
