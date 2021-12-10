const INPUT: &str = include_str!("day10.txt");

fn main() {
    println!("Day 10 Part 1: {:?}", part1(INPUT));
    println!("Day 10 Part 2: {:?}", part2(INPUT));
}

fn score(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

// replace return type as required by the problem
fn part1(input: &str) -> u32 {
    let mut syntax_score = 0;
    for line in input.lines() {
        let mut syntax_checker = vec![];

        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => syntax_checker.push(c),
                ')' | ']' | '}' | '>' => {
                    if let Some(opening) = syntax_checker.pop() {
                        if (opening == '(' && c != ')')
                            || (opening == '[' && c != ']')
                            || (opening == '{' && c != '}')
                            || (opening == '<' && c != '>')
                        {
                            syntax_score += score(c);
                        }
                    }
                }
                _ => (),
            }
        }
    }

    syntax_score
}

fn autocomplete_score(input: &str) -> u32 {
    let mut score = 0;

    for c in input.chars() {
        score *= 5;

        score += match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => 0,
        };
    }

    score
}

// replace return type as required by the problem
fn part2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = include_str!("day10_test.txt");
    use super::*;
    use test_support::test_support::TestCase;

    #[test]
    fn test_part1() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 26397,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part1(*input), *expected);
        }
    }

    #[test]
    fn test_part2() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 288957,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part2(*input), *expected);
        }
    }

    #[test]
    fn test_autocomplete_score() {
        let test_cases = [
            TestCase {
                input: "}}]])})]",
                expected: 288957,
            },
            TestCase {
                input: ")}>]})",
                expected: 5566,
            },
            TestCase {
                input: "}}>}>))))",
                expected: 1480781,
            },
            TestCase {
                input: "]]}}]}]}>",
                expected: 995444,
            },
            TestCase {
                input: "])}>",
                expected: 294,
            },
        ];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(autocomplete_score(*input), *expected);
        }
    }
}
