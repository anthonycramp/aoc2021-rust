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
use std::collections::HashSet;

fn decode_signals(input: &str) -> HashMap<char, char> {
    let mut one: HashSet<char> = HashSet::new();
    let mut four: HashSet<char> = HashSet::new();
    let mut seven: HashSet<char> = HashSet::new();
    let mut eight: HashSet<char> = HashSet::new();
    let mut five_segments: Vec<HashSet<char>> = vec![];
    let mut six_segments: Vec<HashSet<char>> = vec![];

    for signal in input.split_ascii_whitespace() {
        let hashed_signal = signal.chars().collect::<HashSet<char>>();
        match signal.len() {
            2 => one = hashed_signal,
            3 => seven = hashed_signal,
            4 => four = hashed_signal,
            5 => five_segments.push(hashed_signal),
            6 => six_segments.push(hashed_signal),
            7 => eight = hashed_signal,
            _ => (),
        }
    }

    let mut mapping = HashMap::new();

    let a_wire = seven.difference(&one).next().unwrap();
    mapping.insert(*a_wire, 'a');

    let mut three: HashSet<char> = HashSet::new();

    for five_segment in &five_segments {
        if one.is_subset(five_segment) {
            three = five_segment.clone();
            let three_without_one: HashSet<_> = three.difference(&one).copied().collect();
            let d_signal: HashSet<_> = three_without_one.intersection(&four).copied().collect();
            let d_wire = d_signal.iter().next().unwrap();
            mapping.insert(*d_wire, 'd');
            let four_without_one: HashSet<_> = four.difference(&one).copied().collect();
            let b_signal: HashSet<_> = four_without_one.difference(&d_signal).copied().collect();
            let b_wire = b_signal.iter().next().unwrap();
            mapping.insert(*b_wire, 'b');
            break;
        }
    }

    let mut five: HashSet<char> = HashSet::new();

    let abd: HashSet<_> = mapping.keys().copied().collect();
    for five_segment in &five_segments {
        if abd.is_subset(&five_segment) {
            five = five_segment.clone();
            let f_signal: HashSet<_> = five.intersection(&one).copied().collect();
            let f_wire = f_signal.iter().next().unwrap();
            mapping.insert(*f_wire, 'f');
            let mut abdf: HashSet<_> = abd.clone();
            abdf.insert(*f_wire);
            let g_signal: HashSet<_> = five.difference(&abdf).copied().collect();
            let g_wire = g_signal.iter().next().unwrap();
            mapping.insert(*g_wire, 'g');
            let c_signal: HashSet<_> = one.difference(&f_signal).copied().collect();
            let c_wire = c_signal.iter().next().unwrap();
            mapping.insert(*c_wire, 'c');
            break;
        }
    }

    let abcdfg: HashSet<char> = mapping.keys().copied().collect();
    let e_signal: HashSet<_> = eight.difference(&abcdfg).copied().collect();
    let e_wire = e_signal.iter().next().unwrap();
    mapping.insert(*e_wire, 'e');

    mapping
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
    fn test_part2_single_entry() {
        let input = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe";
        let test_cases = [TestCase {
            input: input,
            expected: 8394,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part2(*input), *expected);
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
