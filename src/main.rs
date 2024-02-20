/**
* Game of Life
*
* Rules:
*   1. Any live cell with fewer than two live neighbors dies, as if by underpopulation.
*   2. Any live cell with two or three live neighbors lives on to the next generation.
*   3. Any live cell with more than three live neighbors dies, as if by overpopulation.
*   4. Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
*/
use termion::*;
const WINDOW_WIDTH: u16 = 130;
const WINDOW_HEIGHT: u16 = 40;
const FRAME_DURATION_MS: u128 = 220;
const ALIVE_SYMBOL: char = '█';
const DEAD_SYMBOL: char = ' ';

#[derive(Clone, Copy, PartialEq, Eq)]
enum CellState {
    Alive,
    Dead,
}

#[derive(Clone)]
struct Cell {
    x: u16,
    y: u16,
    state: CellState,
}

fn main() {
    check_terminal_size();
    let mut last_update = std::time::Instant::now();
    let mut board = initialize_board();

    loop {
        update_board(&mut board);
        render_board(&board);
        wait_for_next_frame(last_update);
        last_update = std::time::Instant::now();
    }
}

fn initialize_board() -> Vec<Vec<Cell>> {
    let mut board: Vec<Vec<Cell>> = Vec::new();
    for i in 0..WINDOW_WIDTH {
        let mut row: Vec<Cell> = Vec::new();
        for j in 0..WINDOW_HEIGHT {
            row.push(generate_random_cell(i, j));
        }
        board.push(row);
    }
    board
}

fn update_board(board: &mut Vec<Vec<Cell>>) {
    let board_clone = board.clone();
    for row in board.iter_mut() {
        for cell in row.iter_mut() {
            let num_neighbors = count_neighbors(cell, &board_clone);

            cell.state = match (cell.state, num_neighbors) {
                (CellState::Alive, 2..=3) => CellState::Alive,
                (CellState::Dead, 3) => CellState::Alive,
                _ => CellState::Dead,
            };
        }
    }
}

fn count_neighbors(cell: &mut Cell, board: &[Vec<Cell>]) -> i32 {
    let mut num_neighbors = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }
            let x = cell.x as i16 + i;
            let y = cell.y as i16 + j;
            if x < 0 || y < 0 || x >= WINDOW_WIDTH as i16 || y >= WINDOW_HEIGHT as i16 {
                continue;
            }
            if board[x as usize][y as usize].state == CellState::Alive {
                num_neighbors += 1;
            }
        }
    }
    num_neighbors
}

fn render_board(board: &[Vec<Cell>]) {
    for row in board.iter() {
        for cell in row.iter() {
            let c = match cell.state {
                CellState::Alive => ALIVE_SYMBOL,
                CellState::Dead => DEAD_SYMBOL,
            };
            print_char(cell.x, cell.y, c);
        }
    }
}

fn print_char(x: u16, y: u16, c: char) {
    print!(
        "{}{}{}",
        cursor::Goto(x + 1, y + 1),
        c,
        cursor::Goto(WINDOW_WIDTH + 1, WINDOW_HEIGHT + 1)
    );
}

fn generate_random_cell(x: u16, y: u16) -> Cell {
    let state = if rand::random() && rand::random() {
        CellState::Alive
    } else {
        CellState::Dead
    };
    Cell { x, y, state }
}

fn wait_for_next_frame(last_update: std::time::Instant) {
    let now = std::time::Instant::now();
    let elapsed = now.duration_since(last_update);
    if elapsed.as_millis() < FRAME_DURATION_MS {
        std::thread::sleep(std::time::Duration::from_millis(
            (FRAME_DURATION_MS - elapsed.as_millis()) as u64,
        ));
    }
}

fn check_terminal_size() {
    let (width, height) = termion::terminal_size().unwrap();
    if width < WINDOW_WIDTH || height < WINDOW_HEIGHT {
        panic!(
            "Terminal size too small. Please resize terminal to at least {} by {}",
            WINDOW_WIDTH, WINDOW_HEIGHT
        );
    }
}
