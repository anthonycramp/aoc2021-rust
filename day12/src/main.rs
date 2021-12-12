const INPUT: &str = include_str!("day12.txt");

fn main() {
    println!("Day 12 Part 1: {:?}", part1(INPUT));
    println!("Day 12 Part 2: {:?}", part2(INPUT));
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
enum Cave {
    Big(String),
    Small(String),
}

impl From<&str> for Cave {
    fn from(input: &str) -> Self {
        if input.chars().all(|c| c.is_uppercase()) {
            Cave::Big(String::from(input))
        } else {
            Cave::Small(String::from(input))
        }
    }
}

use std::collections::HashMap;

// replace return type as required by the problem
fn part1(input: &str) -> usize {
    let mut cave_graph: HashMap<Cave, Vec<Cave>> = HashMap::new();

    for line in input.lines() {
        let mut caves = line.split('-');
        let cave1 = Cave::from(caves.next().unwrap());
        let cave2 = Cave::from(caves.next().unwrap());
        cave_graph
            .entry(cave1.clone())
            .or_insert(vec![])
            .push(cave2.clone());
        cave_graph
            .entry(cave2)
            .or_insert(vec![])
            .push(cave1.clone());
    }

    let mut paths: Vec<Vec<Cave>> = vec![];
    let start_cave = Cave::from("start");
    let end_cave = Cave::from("end");

    paths.push(vec![start_cave.clone()]);

    loop {
        if paths.iter().all(|p| p[p.len() - 1] == end_cave) {
            // finish if all paths end at the end_cave
            break;
        }

        let mut new_paths: Vec<Vec<Cave>> = vec![];
        for path in &paths {
            let last_cave_visited = &path[path.len() - 1];
            if *last_cave_visited == end_cave {
                new_paths.push(path.clone());
                continue;
            }

            let connected_caves = cave_graph.get(last_cave_visited).unwrap();
            for connected_cave in connected_caves {
                match connected_cave {
                    Cave::Small(_) => {
                        if !path.contains(connected_cave) {
                            let mut new_path = path.clone();
                            new_path.push(connected_cave.clone());
                            new_paths.push(new_path);
                        }
                    }
                    Cave::Big(_) => {
                        let mut new_path = path.clone();
                        new_path.push(connected_cave.clone());
                        new_paths.push(new_path);
                    }
                }
            }
        }
        paths = new_paths;
    }

    paths.len()
}

// replace return type as required by the problem
fn part2(input: &str) -> usize {
    let mut cave_graph: HashMap<Cave, Vec<Cave>> = HashMap::new();
    let start_cave = Cave::from("start");
    let end_cave = Cave::from("end");

    for line in input.lines() {
        let mut caves = line.split('-');
        let cave1 = Cave::from(caves.next().unwrap());
        let cave2 = Cave::from(caves.next().unwrap());
        cave_graph
            .entry(cave1.clone())
            .or_insert(vec![])
            .push(cave2.clone());
        cave_graph
            .entry(cave2)
            .or_insert(vec![])
            .push(cave1.clone());
    }

    let mut paths: Vec<Vec<Cave>> = vec![];

    paths.push(vec![start_cave.clone()]);

    loop {
        if paths.iter().all(|p| p[p.len() - 1] == end_cave) {
            // finish if all paths end at the end_cave
            break;
        }

        let mut new_paths: Vec<Vec<Cave>> = vec![];
        for path in &paths {
            let last_cave_visited = &path[path.len() - 1];
            if *last_cave_visited == end_cave {
                new_paths.push(path.clone());
                continue;
            }

            let connected_caves = cave_graph.get(last_cave_visited).unwrap();
            for connected_cave in connected_caves {
                if *connected_cave == start_cave {
                    // don't go back to the start cave
                    continue;
                }

                match connected_cave {
                    Cave::Small(_) => {
                        let mut small_cave_counts: HashMap<Cave, usize> = HashMap::new();
                        for cave in path.iter() {
                            if let Cave::Small(_) = cave {
                                *small_cave_counts.entry(cave.clone()).or_insert(0) += 1;
                            }
                        }

                        // add connected_cave to the path if
                        //  1. the connected_cave is not already in the path, or
                        //  2. the connected_cave is in the path once AND there is no other small cave in the path twice
                        if !path.contains(connected_cave)
                            || (small_cave_counts.get(connected_cave).unwrap() == &1
                                && small_cave_counts.values().all(|&v| v < 2))
                        {
                            let mut new_path = path.clone();
                            new_path.push(connected_cave.clone());
                            new_paths.push(new_path);
                        }
                    }
                    Cave::Big(_) => {
                        let mut new_path = path.clone();
                        new_path.push(connected_cave.clone());
                        new_paths.push(new_path);
                    }
                }
            }
        }
        paths = new_paths;
    }

    paths.len()
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = include_str!("day12_test.txt");
    const TEST_INPUT2: &str = include_str!("day12_test2.txt");
    const TEST_INPUT3: &str = include_str!("day12_test3.txt");
    use super::*;
    use test_support::test_support::TestCase;

    #[test]
    fn test_part1_1() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 10,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part1(*input), *expected);
        }
    }

    #[test]
    fn test_part1_2() {
        let test_cases = [TestCase {
            input: TEST_INPUT2,
            expected: 19,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part1(*input), *expected);
        }
    }

    #[test]
    fn test_part1_3() {
        let test_cases = [TestCase {
            input: TEST_INPUT3,
            expected: 226,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part1(*input), *expected);
        }
    }

    #[test]
    fn test_part2_1() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 36,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part2(*input), *expected);
        }
    }

    #[test]
    fn test_part2_2() {
        let test_cases = [TestCase {
            input: TEST_INPUT2,
            expected: 103,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part2(*input), *expected);
        }
    }

    #[test]
    fn test_part2_3() {
        let test_cases = [TestCase {
            input: TEST_INPUT3,
            expected: 3509,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part2(*input), *expected);
        }
    }
}
