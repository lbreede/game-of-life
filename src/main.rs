use rand::Rng;
use raylib::prelude::*;

const SCREEN_WIDTH: i32 = 1000;
const SCREEN_HEIGHT: i32 = 1000;

const CELL_SCALE: i32 = 10;

const BOARD_WIDTH: i32 = SCREEN_WIDTH / CELL_SCALE;
const BOARD_HEIGHT: i32 = SCREEN_HEIGHT / CELL_SCALE;

const CELLS: usize = (BOARD_WIDTH * BOARD_HEIGHT) as usize;

fn random_board(length: usize) -> Vec<bool> {
    let mut rng = rand::rng();
    let mut board: Vec<bool> = Vec::with_capacity(length);
    for _ in 0..length {
        board.push(rng.random_bool(0.1));
    }
    board
}

/// Gets the neighbors of a given index in a given grid.
/// NOTE: This could be precomputed and does not need to be called on every frame!
fn get_neighbors(index: usize, width: usize, height: usize) -> Vec<usize> {
    let x = index % width;
    let y = index / width;

    let mut neighbors = Vec::new();

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue; // skip the cell itself
            }

            let nx = x as isize + dx;
            let ny = y as isize + dy;

            // check bounds
            if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                let n_index = (ny as usize) * width + (nx as usize);
                neighbors.push(n_index);
            }
        }
    }

    neighbors
}

fn draw_cell(d: &mut RaylibDrawHandle, x: i32, y: i32, color: Color) {
    d.draw_rectangle(
        x * CELL_SCALE,
        y * CELL_SCALE,
        1 * CELL_SCALE,
        1 * CELL_SCALE,
        color,
    );
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Conway's Game of Life")
        .build();

    let mut board = random_board(CELLS);

    rl.set_target_fps(24);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        let mut new_board: Vec<bool> = Vec::with_capacity(CELLS);

        for (i, &cell) in board.iter().enumerate() {
            let x = i as i32 % BOARD_WIDTH;
            let y = i as i32 / BOARD_WIDTH;
            let color = if cell { Color::WHITE } else { Color::BLACK };
            draw_cell(&mut d, x, y, color);

            let sum: u8 = get_neighbors(i, BOARD_WIDTH as usize, BOARD_HEIGHT as usize)
                .iter()
                .map(|&n| board[n] as u8)
                .sum();
            new_board.push(if cell { sum == 2 || sum == 3 } else { sum == 3 });
        }

        std::mem::swap(&mut board, &mut new_board);
    }
}
