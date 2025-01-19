use std::collections::HashMap;

use log::info;

use crate::structs::{Battlesnake, Board, Coord};

pub fn process_move(board: &Board, my_snake: &Battlesnake) {
    let w = board.width;
    let h = board.width;

    let my_head = &my_snake.head;
    let my_body = &my_snake.body;

    let grid = board.create_grid(my_snake);

    info!("{:#?}", grid);

    let mut safe_moves = HashMap::from([
        ("up", true),
        ("down", true),
        ("left", true),
        ("right", true),
    ]);

    //check up
    if grid[(my_head.y + 1) as usize][my_head.x as usize].unwrap_or(-1) < 0 {
        safe_moves.insert("up", false);
    }
    //check down
    if grid[(my_head.y - 1) as usize][my_head.x as usize].unwrap_or(-1) < 0 {
        safe_moves.insert("down", false);
    }
    //check left
    if grid[my_head.y as usize][(my_head.x - 1) as usize].unwrap_or(-1) < 0 {
        safe_moves.insert("down", false);
    }
    //check right
    if grid[(my_head.y - 1) as usize][(my_head.x + 1) as usize].unwrap_or(-1) < 0 {
        safe_moves.insert("down", false);
    }
}

#[cfg(test)]
mod pathfinding_tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(true);
    }
}
