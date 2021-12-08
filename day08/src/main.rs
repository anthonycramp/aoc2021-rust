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

use std::collections::HashMap;

fn decode_signals(input: &str) -> HashMap<char, char> {
    HashMap::new()
}

fn read_display(display: &str, mapping: &HashMap<char, char>) -> String {
    let mut display_string = String::with_capacity(display.len());
    for c in display.chars() {
        display_string += &String::from(*mapping.get(&c).unwrap());
    }

    let mut sorting_vec: Vec<char> = display_string.chars().collect();
    sorting_vec.sort_unstable();
    let display_string: String = sorting_vec.into_iter().collect();

    match display_string.as_str() {
        "abcefg" => String::from("0"),
        "cf" => String::from("1"),
        "acdeg" => String::from("2"),
        "acdfg" => String::from("3"),
        "bcdf" => String::from("4"),
        "abdfg" => String::from("5"),
        "abdefg" => String::from("6"),
        "acf" => String::from("7"),
        "abcdefg" => String::from("8"),
        "abcdfg" => String::from("9"),
        _ => panic!(
            "Unknown display string {} mapped to {}",
            display, &display_string
        ),
    }
}

// replace return type as required by the problem
fn part2(input: &str) -> i32 {
    let mut sum_of_all_outputs = 0;

    let lines = input.lines();
    for line in lines {
        let mut input_output = line.split(" | ");
        let input_signals = input_output.next().unwrap();
        let output_segments = input_output.next().unwrap();

        let wire_segment_mapping = decode_signals(input_signals);

        let mut output_value = String::with_capacity(4);
        for output_segment in output_segments.split_ascii_whitespace() {
            output_value += &read_display(output_segment, &wire_segment_mapping);
        }

        let output_value = output_value.parse::<i32>().unwrap();
        sum_of_all_outputs += output_value;
    }

    sum_of_all_outputs
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
