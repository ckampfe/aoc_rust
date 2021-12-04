use std::io::BufRead;

fn main() {
    let input = std::fs::File::open("inputs/4.txt").unwrap();
    let lines = std::io::BufReader::new(input).lines();

    let mut lines = lines.map(|line| line.unwrap());

    let calls = lines.next().unwrap();
    let calls = calls.split(",").map(|s| s.parse::<u8>().unwrap());

    let board_lines: Vec<String> = lines.filter(|line| !line.is_empty()).collect();

    let mut boards = vec![];
    let mut board_line;

    for chunk in board_lines.chunks_exact(5) {
        let mut board = Board::default();
        board_line = 0;

        for line in chunk {
            let line = line.replace("  ", " ");
            let line_numbers = line.trim().split(' ').map(|s| s.parse::<u8>().unwrap());

            for (i, n) in line_numbers.enumerate() {
                board.cells_n[board_line * 5 + i] = n;
            }

            board_line += 1;
        }

        boards.push(board);
    }

    let mut win_order = 0;

    for call in calls {
        for board in &mut boards {
            if board.winning_call.is_none() {
                board.mark_if_appears(call);

                if board.is_win() {
                    win_order += 1;
                    board.mark_won(win_order, call);
                }
            }
        }
    }

    let winning_board = boards
        .iter()
        .find(|board| board.winning_call.unwrap().0 == 100)
        .unwrap();
    let winning_call = winning_board.winning_call.unwrap().1;

    let unmarked_on_winning_board = winning_board.unmarked_numbers();
    let unmarked_sum: usize = unmarked_on_winning_board.iter().map(|n| *n as usize).sum();

    dbg!(winning_board);
    dbg!(unmarked_sum);
    dbg!(winning_call);
    dbg!(unmarked_sum * winning_call as usize);
}

#[derive(Clone, Copy, Debug)]
struct Board {
    cells_n: [u8; 25],
    cells_set: [bool; 25],
    winning_call: Option<(usize, u8)>,
}

impl Board {
    fn mark_if_appears(&mut self, n: u8) {
        if let Some(i) = self.cells_n.iter().position(|cell| *cell == n) {
            self.cells_set[i] = true
        }
    }

    fn is_win(&self) -> bool {
        any_rows_all(self.cells_set) || any_rows_all(transpose(self.cells_set))
    }

    fn mark_won(&mut self, win_order: usize, winning_call: u8) {
        self.winning_call = Some((win_order, winning_call))
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

impl Default for Board {
    fn default() -> Self {
        Self {
            cells_n: Default::default(),
            cells_set: [false; 25],
            winning_call: None,
        }
    }
}

fn any_rows_all(array: [bool; 25]) -> bool {
    array.chunks(5).any(|chunk| chunk.iter().all(|bit| *bit))
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
