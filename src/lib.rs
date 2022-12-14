mod utils;

use js_sys;
use std::collections::VecDeque;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Head,
    Tail,
    Food,
    Empty,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn from_u8(input: u8) -> Option<Direction> {
        if input == 0 {
            return Some(Direction::Up);
        } else if input == 1 {
            return Some(Direction::Down);
        } else if input == 2 {
            return Some(Direction::Left);
        } else if input == 3 {
            return Some(Direction::Right);
        } else {
            return None;
        }
    }

    pub fn inverse(&self) -> Direction{
        match self{
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

type Coordiantes = (u32, u32);

#[wasm_bindgen]
pub struct ChangedCells {
    xs: Vec<u32>,
    ys: Vec<u32>,
    cells: Vec<u8>,
    len: u32,
    score: usize,
}

#[wasm_bindgen]
impl ChangedCells {
    #[wasm_bindgen(getter)]
    pub fn xs(&self) -> js_sys::Uint32Array {
        return js_sys::Uint32Array::from(&self.xs[..]);
    }

    #[wasm_bindgen(getter)]
    pub fn ys(&self) -> js_sys::Uint32Array {
        js_sys::Uint32Array::from(&self.ys[..])
    }

    #[wasm_bindgen(getter)]
    pub fn cells(&self) -> js_sys::Uint8Array {
        js_sys::Uint8Array::from(&self.cells[..])
    }

    #[wasm_bindgen(getter)]
    pub fn len(&self) -> u32 {
        self.len
    }

    #[wasm_bindgen(getter)]
    pub fn score(&self) -> usize {
        self.score
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    direction: Direction,
    snake: VecDeque<Coordiantes>, 
}

#[wasm_bindgen]
impl Universe {
    fn get_index(&self, coordinates: Coordiantes) -> usize {
        let (row, column) = coordinates;
        (row * self.width + column) as usize
    }

    fn get_head_coordinates(&self) -> (u32, u32) {
        self.snake.front().unwrap().clone()
    }

    fn next_move_inbounds(&self, coordinates: Coordiantes, direction: Direction) -> bool {
        let (row, col) = coordinates;
        return match direction {
            Direction::Up => {
                if row == 0 {
                    return false;
                }
                true
            }
            Direction::Down => {
                if row == self.height - 1 {
                    return false;
                }
                true
            }
            Direction::Left => {
                if col == 0 {
                    return false;
                }
                true
            }
            Direction::Right => {
                if col == self.width - 1 {
                    return false;
                }
                true
            }
        };
    }

    fn next_head(&self, direction: Direction) -> Option<Coordiantes> {
        if !self.next_move_inbounds(self.get_head_coordinates(), direction) {
            return None;
        }

        let (head_row, head_column) = self.get_head_coordinates();

        match direction {
            Direction::Up => Some((head_row - 1, head_column)),
            Direction::Down => Some((head_row + 1, head_column)),
            Direction::Left => Some((head_row, head_column - 1)),
            Direction::Right => Some((head_row, head_column + 1)),
        }
    }

    fn random_cell_for_food(&self) -> Coordiantes {
        let r = js_sys::Math::random();
        let index = ((self.width * self.height) as f64 * r) as u32;
        let coordinates = (index / self.width, index % self.width);
        match self.cells[index as usize] {
            Cell::Empty => coordinates,
            _ => self.random_cell_for_food(),
        }
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        utils::set_panic_hook();

        let width: u32 = 64;
        let height: u32 = 64;
        let head_coordinates = (5, 20);

        let head_index = head_coordinates.0 * 64 + head_coordinates.1;
        let food_index = 5 * 64 + 21;
        let cells = (0..width * height)
            .map(|i| {
                if i == head_index {
                    return Cell::Head;
                }

                if i == food_index {
                    return Cell::Food;
                }
                Cell::Empty
            })
            .collect();

        let mut snake = VecDeque::with_capacity((width * height) as usize);
        snake.push_front(head_coordinates);
        let direction = Direction::Right;

        Universe {
            width,
            height,
            cells,
            direction,
            snake,
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn tick(&mut self, key_input: Option<u8>) -> Option<ChangedCells> {
        let direction = {
            let input_direction = Direction::from_u8(key_input.unwrap_or(4));
            match input_direction {
                Some(input_direction) if self.cells.len() > 1  && input_direction == self.direction.inverse() =>  self.direction,
                Some(input_direction) => input_direction,
                None => self.direction
            }
        };

        self.direction = direction;
        let next_head =  match self.next_head(self.direction) {
            Some(next) => next,
            None => return None
        };

        

        let next_head_index = self.get_index(next_head);
        let next_head_cell = &self.cells[next_head_index];

        return match next_head_cell {
            Cell::Food => {
                //add head
                let old_head = self.snake.front().unwrap().clone();
                self.snake.push_front(next_head);

                //update old head
                let old_head_index = self.get_index(old_head);
                self.cells[old_head_index] = Cell::Tail;

                //update new head
                self.cells[next_head_index] = Cell::Head;

                //update food
                let new_food_coordinates = self.random_cell_for_food();
                let new_food_idx = self.get_index(new_food_coordinates);
                self.cells[new_food_idx] = Cell::Food;

                Some(ChangedCells {
                    xs: vec![next_head.0, old_head.0, new_food_coordinates.0],
                    ys: vec![next_head.1, old_head.1, new_food_coordinates.1],
                    cells: vec![Cell::Head as u8, Cell::Tail as u8, Cell::Food as u8],
                    len: 3,
                    score: self.snake.len()
                })
            }
            Cell::Empty => {
                //clear tail
                let cell_to_empty = self.snake.pop_back().unwrap();
                let cell_to_empty_index = self.get_index(cell_to_empty);
                self.cells[cell_to_empty_index] = Cell::Empty;

                //move head
                self.snake.push_front(next_head);
                let next_head_index = self.get_index(next_head);
                self.cells[next_head_index] = Cell::Head;

                Some(ChangedCells {
                    xs: vec![next_head.0, cell_to_empty.0],
                    ys: vec![next_head.1, cell_to_empty.1],
                    cells: vec![Cell::Head as u8, Cell::Empty as u8],
                    len: 2,
                    score: self.snake.len(),
                })
            }
            _ => None,
        };
    }
}
