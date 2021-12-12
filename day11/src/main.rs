const INPUT: &str = include_str!("day11.txt");

fn main() {
    println!("Day 11 Part 1: {:?}", part1(INPUT));
    println!("Day 11 Part 2: {:?}", part2(INPUT));
}

fn get_neighbours(loc: usize) -> Vec<usize> {
    let grid_width = 10;
    let grid_size = grid_width * grid_width;

    let upper_left_loc = loc.checked_sub(grid_width + 1);
    let upper_loc = loc.checked_sub(grid_width);
    let upper_right_loc = loc.checked_sub(grid_width - 1);
    let right_loc = loc + 1;
    let lower_loc = loc + grid_width;
    let lower_right_loc = lower_loc + 1;
    let lower_left_loc = lower_loc.checked_sub(1);
    let left_loc = loc.checked_sub(1);

    let mut neighbours = vec![];

    if loc == 0 {
        neighbours.push(right_loc);
        neighbours.push(lower_right_loc);
        neighbours.push(lower_loc);
    } else if loc == 9 {
        // upper right corner
        neighbours.push(left_loc.unwrap());
        neighbours.push(lower_left_loc.unwrap());
        neighbours.push(lower_loc);
    } else if loc == 90 {
        // lower left corner
        neighbours.push(upper_loc.unwrap());
        neighbours.push(upper_right_loc.unwrap());
        neighbours.push(right_loc);
    } else if loc == 99 {
        // lower right corner
        neighbours.push(left_loc.unwrap());
        neighbours.push(upper_left_loc.unwrap());
        neighbours.push(upper_loc.unwrap());
    } else if loc % grid_width == 0 {
        // left edge
        neighbours.push(upper_loc.unwrap());
        neighbours.push(upper_right_loc.unwrap());
        neighbours.push(right_loc);
        neighbours.push(lower_right_loc);
        neighbours.push(lower_loc);
    } else if (loc + 1) % grid_width == 0 {
        // right edge
        neighbours.push(upper_loc.unwrap());
        neighbours.push(lower_loc);
        neighbours.push(lower_left_loc.unwrap());
        neighbours.push(left_loc.unwrap());
        neighbours.push(upper_left_loc.unwrap());
    } else if loc < 9 {
        // top edge
        neighbours.push(left_loc.unwrap());
        neighbours.push(right_loc);
        neighbours.push(lower_right_loc);
        neighbours.push(lower_loc);
        neighbours.push(lower_left_loc.unwrap());
    } else if loc > 90 {
        // bottom edge
        neighbours.push(left_loc.unwrap());
        neighbours.push(upper_left_loc.unwrap());
        neighbours.push(upper_loc.unwrap());
        neighbours.push(upper_right_loc.unwrap());
        neighbours.push(right_loc);
    } else {
        neighbours.push(left_loc.unwrap());
        neighbours.push(upper_left_loc.unwrap());
        neighbours.push(upper_loc.unwrap());
        neighbours.push(upper_right_loc.unwrap());
        neighbours.push(right_loc);
        neighbours.push(lower_right_loc);
        neighbours.push(lower_loc);
        neighbours.push(lower_left_loc.unwrap());
    }

    neighbours
}

fn print_grid(grid: &[u32]) {
    for i in 0..100 {
        if i % 10 == 0 {
            println!();
        }

        print!("{:2}", grid[i]);
    }
    println!();
}

// replace return type as required by the problem
fn part1(input: &str) -> u32 {
    let mut grid = vec![];
    for line in input.lines() {
        for c in line.chars() {
            grid.push(c.to_digit(10).unwrap());
        }
    }

    let mut flash_count: usize = 0;

    for i in 0..100 {
        for octopus_energy in &mut grid {
            *octopus_energy += 1;
        }

        // get the locations of the octopi who are ready to flash
        let mut frontier: Vec<usize> = grid
            .iter()
            .enumerate()
            .filter(|&(_, e)| *e == 10)
            .map(|(i, _)| i)
            .collect();

        let mut flashed: Vec<usize> = vec![];

        while let Some(flash_location) = frontier.pop() {
            // for each octopus in the frontier
            //      get its neighbour locations
            let neighbours = get_neighbours(flash_location);
            for neighbour_location in neighbours.into_iter() {
                if grid[neighbour_location] < 10 {
                    //      increment each neighbour location by 1 unless the energy at that location is 10
                    grid[neighbour_location] += 1;

                    if grid[neighbour_location] == 10
                        && !frontier.contains(&neighbour_location)
                        && !flashed.contains(&neighbour_location)
                    {
                        frontier.push(neighbour_location);
                    }
                }
            }

            //      add to flashed
            flashed.push(flash_location);
        }

        // set energy for flashed octopi to 0
        for flash_location in &flashed {
            grid[*flash_location] = 0;
        }

        // increment flashed_count by size of the flashed container
        flash_count += flashed.len();
        // clear the flashed container
        flashed.clear();
    }

    flash_count as u32
}

// replace return type as required by the problem
fn part2(input: &str) -> u32 {
    let mut grid = vec![];
    for line in input.lines() {
        for c in line.chars() {
            grid.push(c.to_digit(10).unwrap());
        }
    }

    for i in 0.. {
        for octopus_energy in &mut grid {
            *octopus_energy += 1;
        }

        // get the locations of the octopi who are ready to flash
        let mut frontier: Vec<usize> = grid
            .iter()
            .enumerate()
            .filter(|&(_, e)| *e == 10)
            .map(|(i, _)| i)
            .collect();

        let mut flashed: Vec<usize> = vec![];

        while let Some(flash_location) = frontier.pop() {
            // for each octopus in the frontier
            //      get its neighbour locations
            let neighbours = get_neighbours(flash_location);
            for neighbour_location in neighbours.into_iter() {
                if grid[neighbour_location] < 10 {
                    //      increment each neighbour location by 1 unless the energy at that location is 10
                    grid[neighbour_location] += 1;

                    if grid[neighbour_location] == 10
                        && !frontier.contains(&neighbour_location)
                        && !flashed.contains(&neighbour_location)
                    {
                        frontier.push(neighbour_location);
                    }
                }
            }

            //      add to flashed
            flashed.push(flash_location);
        }

        // set energy for flashed octopi to 0
        for flash_location in &flashed {
            grid[*flash_location] = 0;
        }

        if flashed.len() == grid.len() {
            return i + 1;
        }
        flashed.clear();
    }

    0
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = include_str!("day11_test.txt");
    use super::*;
    use test_support::test_support::TestCase;

    #[test]
    fn test_part1() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 1656,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part1(*input), *expected);
        }
    }

    #[test]
    fn test_part2() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 195,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part2(*input), *expected);
        }
    }
}
