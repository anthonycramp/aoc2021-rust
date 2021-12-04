const INPUT: &str = include_str!("day04.txt");

fn main() {
    println!("Day NN Part 1: {:?}", part1(INPUT));
    println!("Day NN Part 2: {:?}", part2(INPUT));
}

#[derive(Debug, Clone)]
struct BingoBoard {
    board: Vec<i32>,
    complete: bool,
}

impl BingoBoard {
    fn mark(&mut self, called_num: i32) {
        if !self.is_complete() {
            for num in self.board.iter_mut() {
                if *num == called_num {
                    *num = 0;
                }
            }
            self.check_complete();
        }
    }

    fn check_row(&self, index: usize) -> bool {
        self.board[index * 5]
            + self.board[index * 5 + 1]
            + self.board[index * 5 + 2]
            + self.board[index * 5 + 3]
            + self.board[index * 5 + 4]
            == 0
    }

    fn check_rows(&self) -> bool {
        self.check_row(0)
            || self.check_row(1)
            || self.check_row(2)
            || self.check_row(3)
            || self.check_row(4)
    }

    fn check_column(&self, index: usize) -> bool {
        self.board[index]
            + self.board[index + 5]
            + self.board[index + 10]
            + self.board[index + 15]
            + self.board[index + 20]
            == 0
    }

    fn check_columns(&self) -> bool {
        self.check_column(0)
            || self.check_column(1)
            || self.check_column(2)
            || self.check_column(3)
            || self.check_column(4)
    }

    fn check_complete(&mut self) {
        self.complete = self.check_rows() || self.check_columns()
    }

    fn is_complete(&self) -> bool {
        self.complete
    }

    fn sum(&self) -> i32 {
        self.board.iter().sum::<i32>()
    }
}

// replace return type as required by the problem
fn part1(input: &str) -> i32 {
    let lines: Vec<String> = input.lines().map(String::from).collect();
    let called_numbers: Vec<i32> = lines[0].split(",").map(|n| n.parse().unwrap()).collect();
    let mut bingo_boards = vec![];

    let mut board_index = 2;
    loop {
        if board_index >= lines.len() {
            break;
        }

        let mut board = BingoBoard {
            board: vec![],
            complete: false,
        };

        for i in board_index..(board_index + 5) {
            for num in lines[i].split_ascii_whitespace() {
                board.board.push(num.parse().unwrap());
            }
        }

        bingo_boards.push(board);

        board_index += 6;
    }

    for called_num in called_numbers {
        for board in bingo_boards.iter_mut() {
            board.mark(called_num);
            if board.is_complete() {
                return board.sum() * called_num;
            }
        }
    }
    0
}

// replace return type as required by the problem
fn part2(input: &str) -> i32 {
    let lines: Vec<String> = input.lines().map(String::from).collect();
    let called_numbers: Vec<i32> = lines[0].split(",").map(|n| n.parse().unwrap()).collect();
    let mut bingo_boards = vec![];

    let mut board_index = 2;
    loop {
        if board_index >= lines.len() {
            break;
        }

        let mut board = BingoBoard {
            board: vec![],
            complete: false,
        };

        for i in board_index..(board_index + 5) {
            for num in lines[i].split_ascii_whitespace() {
                board.board.push(num.parse().unwrap());
            }
        }

        bingo_boards.push(board);

        board_index += 6;
    }

    let mut last_board_won = BingoBoard {
        board: vec![],
        complete: false,
    };
    let mut last_called_number_on_win: i32 = 0;
    for called_num in called_numbers {
        for board in bingo_boards.iter_mut() {
            if !board.is_complete() {
                board.mark(called_num);
                if board.is_complete() {
                    last_board_won = board.clone();
                    last_called_number_on_win = called_num;
                }
            }
        }
    }
    last_board_won.sum() * last_called_number_on_win
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = include_str!("day04_test.txt");
    use super::*;
    use test_support::test_support::TestCase;

    #[test]
    fn test_part1() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 4512,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part1(*input), *expected);
        }
    }

    #[test]
    fn test_part2() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 1924,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part2(*input), *expected);
        }
    }
}
