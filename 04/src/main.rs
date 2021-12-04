use std::collections::HashSet;

struct Board {
    num_rows: usize,
    num_cols: usize,
    cells: Vec<u32>,
    results: Vec<bool>,
}

impl Board {
    fn new(cells: Vec<u32>, num_rows: usize) -> Self {
        let results = vec![false; cells.len()];
        let num_cols = results.len() / num_rows;
        Board {
            num_rows,
            num_cols,
            cells,
            results,
        }
    }

    fn mark(&mut self, number: u32) -> bool {
        let index = self.cells
            .iter()
            .position(|cell| *cell == number);

        let index = match index {
            Some(index) => index,
            None => return false,
        };

        self.results[index] = true;

        let col = index % self.num_cols;
        let row = index / self.num_rows;

        self.test_col(col) || self.test_row(row)
    }

    fn test_row(&self, row: usize) -> bool {
        let start = row * self.num_cols;
        let end = start + self.num_cols;
        let mut filled = true;
        for index in start..end {
            filled = filled && self.results[index];
        }
        filled
    }

    fn test_col(&self, col: usize) -> bool {
        let mut filled = true;
        for i in 0..self.num_rows {
            let index = self.num_cols * i + col;
            filled = filled && self.results[index];
        }
        filled
    }

    fn unmarked_total(&self) -> u32 {
        self.results
            .iter()
            .enumerate()
            .filter(|(_i, marked)| !**marked)
            .fold(0, |acc, (i, _marked)| acc + self.cells[i])
    }

    fn clear(&mut self) {
        self.results.fill(false);
    }
}

struct Bingo {
    numbers: Vec<u32>,
    boards: Vec<Board>,
}

impl Bingo {
    fn clear_boards(&mut self) {
        self.boards.iter_mut().for_each(|board| board.clear());
    }
}

fn get_data() -> Bingo {
    let input_str = include_str!("./input.txt");
    let mut input_iter = input_str.lines();

    let numbers: Vec<u32> = input_iter
        .next()
        .unwrap()
        .split(',')
        .map(|number_str| number_str.parse().unwrap())
        .collect();

    input_iter.next();

    let mut boards: Vec<Board> = Vec::new();
    let mut rows = 0;
    let mut board_cells = Vec::new();
    for line in input_iter {
        match line {
            "" => {
                boards.push(Board::new(board_cells, rows));
                board_cells = Vec::new();
                rows = 0;
            }
            _ => {
                line
                    .split_whitespace()
                    .for_each(|number_str| {
                        let number = number_str.parse().unwrap();
                        board_cells.push(number);
                    });
                rows += 1;
            }
        }
    }

    boards.push(Board::new(board_cells, rows));
    Bingo { numbers, boards }
}

fn part1(bingo: &mut Bingo) {
    for number in &bingo.numbers {
        for board in bingo.boards.iter_mut() {
            if board.mark(*number) {
                println!("First BINGO! score: {}", number * board.unmarked_total());
                return
            }
        }
    }
}

fn part2(bingo: &mut Bingo) {
    let num_boards = bingo.boards.len();
    let mut complete = HashSet::new();

    for number in &bingo.numbers {
        for (i, board) in bingo.boards.iter_mut().enumerate() {
            if complete.contains(&i) { continue; }

            if board.mark(*number) {
                complete.insert(i);

                if complete.len() == num_boards {
                    println!("Last BINGO! score: {}", number * board.unmarked_total());
                    return;
                }
            }
        }
    }
}

fn main() {
    let mut data = get_data();
    part1(&mut data);
    data.clear_boards();
    part2(&mut data);
}
