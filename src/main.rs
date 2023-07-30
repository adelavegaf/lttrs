use std::{cmp, collections::HashSet};

use anyhow::Result;
use rand::{Rng, RngCore};

fn main() -> Result<()> {
    let mut rng = rand::thread_rng();
    let board = create_board(&mut rng);
    print_board(&board);

    let vocab = std::fs::read_to_string("./data/twl06.txt")?
        .split('\n')
        .map(String::from)
        .collect();

    let words = find_words(&board, &vocab);

    println!("{:#?}", words);

    Ok(())
}

const BOARD_SIZE: usize = 4;
type Board = [[u8; BOARD_SIZE]; BOARD_SIZE];

fn create_board(rng: &mut impl RngCore) -> Board {
    core::array::from_fn(|_| core::array::from_fn(|_| rng.gen_range(b'A'..=b'Z')))
}

fn print_board(board: &Board) {
    for row in board {
        println!("{}", std::str::from_utf8(row).unwrap());
    }
}

fn find_words(board: &Board, vocab: &HashSet<String>) -> HashSet<String> {
    let mut words = HashSet::new();
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            let pos = (i, j);
            let visited = [[false; BOARD_SIZE]; BOARD_SIZE];
            find_words_starting_at(pos, board, vocab, visited, String::new(), &mut words);
        }
    }
    words
}

fn find_words_starting_at(
    pos: (usize, usize),
    board: &Board,
    vocab: &HashSet<String>,
    mut visited: [[bool; 4]; 4],
    mut cur_word: String,
    results: &mut HashSet<String>,
) {
    if visited[pos.0][pos.1] {
        return;
    }

    visited[pos.0][pos.1] = true;
    cur_word.push(board[pos.0][pos.1] as char);

    if vocab.contains(&cur_word) {
        results.insert(cur_word.clone());
    }

    let min_row = pos.0.checked_add_signed(-1).unwrap_or(0);
    let max_row = cmp::min(pos.0 + 1, BOARD_SIZE - 1);
    let min_col = pos.1.checked_add_signed(-1).unwrap_or(0);
    let max_col = cmp::min(pos.1 + 1, BOARD_SIZE - 1);

    for i in min_row..=max_row {
        for j in min_col..=max_col {
            let pos = (i, j);
            // [[bool; 4]; 4] implements copy so no need to be explicit with the clone
            find_words_starting_at(pos, board, vocab, visited, cur_word.clone(), results);
        }
    }
}
