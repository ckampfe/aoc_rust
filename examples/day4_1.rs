use std::io::BufRead;

fn main() {
    let input = std::fs::File::open("inputs/4.txt").unwrap();
    let lines = std::io::BufReader::new(input).lines();

    let mut lines = lines.map(|line| line.unwrap());

    let calls: Vec<u8> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect();

    dbg!(calls.len());

    lines.next();

    let mut boards = vec![];
    let mut board_line = 0;

    let mut current_board = Board::default();

    for line in lines {
        if line.is_empty() {
            boards.push(current_board);
            current_board = Board::default();
            board_line = 0;
            continue;
        }

        let line = line.replace("  ", " ");
        let line_numbers = line.trim().split(' ').map(|s| s.parse::<u8>().unwrap());

        for (i, n) in line_numbers.enumerate() {
            current_board.cells_n[board_line * 5 + i] = n;
        }

        board_line += 1;
    }

    let mut winning_call = 0;
    let mut winning_board = Board::default();

    'calls: for call in calls {
        for board in &mut boards {
            board.mark_if_appears(call);

            if board.is_win() {
                winning_call = call;
                winning_board = *board;
                break 'calls;
            }
        }
    }

    let unmarked_on_winning_board = winning_board.unmarked_numbers();
    let unmarked_sum: usize = unmarked_on_winning_board.iter().map(|n| *n as usize).sum();

    dbg!(winning_call);
    dbg!(winning_board);
    dbg!(unmarked_sum);
    dbg!(unmarked_sum * winning_call as usize);
}

#[derive(Clone, Copy, Debug, Default)]
struct Board {
    cells_n: [u8; 25],
    cells_set: [bool; 25],
}

impl Board {
    fn mark_if_appears(&mut self, n: u8) {
        if let Some(i) = self.cells_n.iter().position(|cell| *cell == n) {
            self.cells_set[i] = true
        }
    }

    fn is_win(&self) -> bool {
        Board::any_rows_all(self.cells_set) || Board::any_rows_all(transpose(self.cells_set))
    }

    fn any_rows_all(array: [bool; 25]) -> bool {
        array.chunks(5).any(|chunk| chunk.iter().all(|bit| *bit))
    }

    fn unmarked_numbers(&self) -> Vec<u8> {
        let mut unmarked = vec![];

        for (i, is_set) in self.cells_set.iter().enumerate() {
            if !is_set {
                unmarked.push(self.cells_n[i])
            }
        }

        unmarked
    }
}

fn transpose<T: Copy + Default>(a: [T; 25]) -> [T; 25] {
    let mut out: [T; 25] = Default::default();

    for y in 0..5 {
        for x in 0..5 {
            out[y + x * 5] = a[x + y * 5];
        }
    }

    out
}
