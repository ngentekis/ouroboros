use std::collections::HashMap;

use log::info;

use crate::structs::{Battlesnake, Board};

pub fn process_move(board: &Board, my_snake: &Battlesnake) -> String {
    let w = board.width;
    let h = board.width;

    let my_head = &my_snake.head;
    let _my_body = &my_snake.body;

    let grid = board.create_grid(my_snake);

    info!("{:#?}", grid);

    let mut safe_moves = HashMap::from([
        ("up", true),
        ("down", true),
        ("left", true),
        ("right", true),
    ]);

    let current_direction = if my_head.y + 1 < w
        && grid[(my_head.y + 1) as usize][my_head.x as usize].unwrap_or(i32::MIN) == -1
    {
        String::from("up")
    } else if my_head.y > 0
        && grid[(my_head.y - 1) as usize][my_head.x as usize].unwrap_or(i32::MIN) == -1
    {
        String::from("down")
    } else if my_head.x > 0
        && grid[my_head.y as usize][(my_head.x - 1) as usize].unwrap_or(i32::MIN) == -1
    {
        String::from("right")
    } else {
        String::from("left")
    };

    //check up
    if my_head.y + 1 < w
        && grid[(my_head.y + 1) as usize][my_head.x as usize].unwrap_or(i32::MIN) < 0
    {
        safe_moves.insert("up", false);
    }
    //check down
    if my_head.y > 0 && grid[(my_head.y - 1) as usize][my_head.x as usize].unwrap_or(i32::MIN) < 0 {
        safe_moves.insert("down", false);
    }
    //check left
    if my_head.x > 0 && grid[my_head.y as usize][(my_head.x - 1) as usize].unwrap_or(i32::MIN) < 0 {
        safe_moves.insert("down", false);
    }
    //check right
    if my_head.x + 1 < h
        && grid[(my_head.y) as usize][(my_head.x + 1) as usize].unwrap_or(i32::MIN) < 0
    {
        safe_moves.insert("down", false);
    }

    current_direction
}

#[cfg(test)]
mod pathfinding_tests {
    use crate::structs::Coord;

    use super::*;

    #[test]
    fn it_works() {
        assert!(true);
    }

    fn get_snake() -> Battlesnake {
        Battlesnake {
            id: String::from("test"),
            name: String::from("test"),
            health: 100,
            body: vec![],
            head: Coord { x: 0, y: 0 },
            length: 1,
            latency: String::new(),
            shout: Some(String::new()),
        }
    }

    fn get_board() -> Board {
        Board {
            food: vec![],
            hazards: vec![],
            snakes: vec![],
            height: 11,
            width: 11,
        }
    }

    #[test]
    fn get_move() {
        let snake = get_snake();
        let board = get_board();

        let actual = process_move(&board, &snake);

        assert_eq!(actual, "up".to_string());
    }
}
