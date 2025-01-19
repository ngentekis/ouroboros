use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Debug)]
pub struct Game {
    pub id: String,
    pub ruleset: HashMap<String, Value>,
    pub timeout: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Board {
    pub food: Vec<Coord>,
    pub hazards: Vec<Coord>,
    pub snakes: Vec<Battlesnake>,
    pub height: i32,
    pub width: i32,
}

impl Board {
    pub fn create_grid(self: &Self, my_snake: &Battlesnake) -> Vec<Vec<Option<i32>>> {
        let mut grid = vec![vec![Some(0); self.width as usize]; self.height as usize];

        my_snake
            .body
            .iter()
            .for_each(|c| grid[c.y as usize][c.x as usize] = Some(-1));

        self.food
            .iter()
            .for_each(|c| grid[c.y as usize][c.x as usize] = Some(1));

        self.hazards
            .iter()
            .for_each(|c| grid[c.y as usize][c.x as usize] = Some(-2));

        for snake in self.snakes.iter() {
            snake
                .body
                .iter()
                .for_each(|c| grid[c.y as usize][c.x as usize] = Some(-3));
        }

        grid
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GameState {
    pub game: Game,
    pub turn: u32,
    pub board: Board,
    pub you: Battlesnake,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Battlesnake {
    pub id: String,
    pub name: String,
    pub health: i32,
    pub body: Vec<Coord>,
    pub head: Coord,
    pub length: u32,
    pub latency: String,
    pub shout: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

#[cfg(test)]
mod structs_tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(true);
    }

    #[test]
    fn test_board_create_empty_grid() {
        let height: i32 = 11;
        let width: i32 = 11;

        let snake = Battlesnake {
            id: String::from("test"),
            name: String::from("test"),
            health: 100,
            body: vec![],
            head: Coord { x: 0, y: 0 },
            length: 1,
            latency: String::new(),
            shout: Some(String::new()),
        };

        let expected = vec![vec![Some(0); height as usize]; width as usize];

        let board = Board {
            food: vec![],
            hazards: vec![],
            snakes: vec![],
            height,
            width,
        };

        let grid = board.create_grid(&snake);

        assert_eq!(expected, grid)
    }

    #[test]
    fn test_board_create_non_empty_grid() {
        let height: i32 = 5;
        let width: i32 = 5;

        let snake = Battlesnake {
            id: String::from("test"),
            name: String::from("test"),
            health: 100,
            body: vec![
                Coord { x: 3, y: 4 },
                Coord { x: 2, y: 4 },
                Coord { x: 1, y: 4 },
                Coord { x: 1, y: 3 },
                Coord { x: 1, y: 2 },
                Coord { x: 1, y: 1 },
            ],
            head: Coord { x: 3, y: 4 },
            length: 6,
            latency: String::new(),
            shout: Some(String::new()),
        };

        let row_0 = vec![Some(1), Some(0), Some(0), Some(0), Some(0)];
        let row_1 = vec![Some(0), Some(-1), Some(0), Some(0), Some(0)];
        let row_2 = vec![Some(0), Some(-1), Some(-2), Some(0), Some(0)];
        let row_3 = vec![Some(0), Some(-1), Some(0), Some(0), Some(0)];
        let row_4 = vec![Some(0), Some(-1), Some(-1), Some(-1), Some(0)];

        let expected = vec![row_0, row_1, row_2, row_3, row_4];

        let board = Board {
            food: vec![Coord { x: 0, y: 0 }],
            hazards: vec![Coord { x: 2, y: 2 }],
            snakes: vec![],
            height,
            width,
        };

        let grid = board.create_grid(&snake);

        assert_eq!(expected, grid)
    }
}
