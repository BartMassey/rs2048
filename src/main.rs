mod board;
mod direction;
mod game;

use crate::board::Board;
use crate::direction::Direction;
use crate::game::Game;
use pancurses::{curs_set, half_delay, initscr, noecho, Input};

fn main() {
    let mut game: Game = Game::new();
    game.init();
    let window = initscr();
    window.printw(game.to_string());
    window.refresh();
    noecho();
    curs_set(0);
    let mut progress: bool = false;
    loop {
        progress = false;
        match window.getch() {
            Some(Input::Character('w')) => progress = game.step(&Direction::Up),
            Some(Input::Character('s')) => progress = game.step(&Direction::Down),
            Some(Input::Character('a')) => progress = game.step(&Direction::Left),
            Some(Input::Character('d')) => progress = game.step(&Direction::Right),
            None => {}
            Some(_) => break,
        }
        if progress {
            game.generate_new_cell();
            half_delay(200);
            window.clear();
            window.printw(game.to_string());
            window.refresh();
        }
    }
}

mod tests {
    use super::*;

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn test_game_step() {
        let mut game: Game = Game::new();
        let mut before: Vec<Vec<usize>> = Vec::new();
        let mut direction: Vec<&Direction> = Vec::new();
        let mut after: Vec<Vec<usize>> = Vec::new();

        // test 0
        before.push(vec![  2,   0,   0,   0,
                           2,   4,   0,   0,
                           8,   0,   0,   2,
                           8,  16,   4,   0]);
        direction.push(&Direction::Down);
        after.push(vec![   0,   0,   0,   0,
                           0,   0,   0,   0,
                           4,   4,   0,   0,
                           16,  16,   4,   2]);

        // test 1
        before.push(vec![  0,   0,   0,   0,
                           0,   2,   0,   4,
                           0,   0,   0,   4,
                           0,   0,   2,   4]);
        direction.push(&Direction::Down);
        after.push(vec![  0,   0,   0,   0,
                          0,   0,   0,   0,
                          0,   0,   0,   4,
                          0,   2,   2,   8]);

        // test 2
        before.push(vec![  2,   2,   0,   2,
                           8,   0,   0,   0,
                           4,   0,   4,   0,
                           0,   0,   0,   0]);
        direction.push(&Direction::Left);
        after.push(vec![  4,   2,   0,   0,
                          8,   0,   0,   0,
                          8,   0,   0,   0,
                          0,   0,   0,   0]);

        // test 3
        before.push(vec![  2,   2,   4,   2,
                           2,   4,   0,   8,
                           0,   2,   0,  16,
                           0,   0,   0,   0]);
        direction.push(&Direction::Left);
        after.push(vec![  4,   4,   2,   0,
                          2,   4,   8,   0,
                          2,   16,   0,  0,
                          0,   0,   0,   0]);
        
        // test 4
        before.push(vec![  4,   2,   0,   0,
                           0,   2,   0,   0,
                           2,   4,   0,   0,
                           2,   4,   2,   0]);
        direction.push(&Direction::Up);
        after.push(vec![  4,   4,   2,   0,
                          4,   8,   0,   0,
                          0,   0,   0,   0,
                          0,   0,   0,   0]);

        // test 5
        before.push(vec![  0,   0,   2,   2,
                           0,   0,   0,   0,
                           0,   0,   8,   2,
                           0,   4,   8,   4]);
        direction.push(&Direction::Up);
        after.push(vec![  0,   4,   2,   4,
                          0,   0,  16,   4,
                          0,   0,   0,   0,
                          0,   0,   0,   0]);

        // test 6
        before.push(vec![  4,   8,   4,   4,
                           2,   2,   0,   0,
                           4,   2,   0,   0,
                           2,   0,   0,   0]);
        direction.push(&Direction::Right);
        after.push(vec![  0,   4,   8,   8,
                          0,   0,   0,   4,
                          0,   0,   4,   2,
                          0,   0,   0,   2]);

        // test 7
        before.push(vec![ 16,   4,   2,   2,
                           2,   8,   4,   4,
                           4,   0,   2,   0,
                           0,   2,   0,   0]);
        direction.push(&Direction::Right);
        after.push(vec![  0,  16,   4,   4,
                          0,   2,   8,   8,
                          0,   0,   4,   2,
                          0,   0,   0,   2]);
        
        for i in 0..before.len() {
            game.set_states(before[i].clone());
            game.step(direction[i]);
            assert_eq!(game.get_states(), after[i]);
        }
        
    }
}