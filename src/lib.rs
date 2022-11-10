mod utils;


use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-snake!");
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Head,
    Tail,
    Food,
    Empty
}

pub enum Direction{
    Up,
    Down,
    Left,
    Right,
}

type Coordiantes = (u32, u32);

#[wasm_bindgen]
pub struct ChagnedCell {
    coordinate: Coordiantes,
    newCell: Cell
}

#[wasm_bindgen]
pub struct Universe{
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    direction: Direction,
    snake: Vec<Coordiantes>, //list of snake cell coordinates w/ head at 0
}

impl Universe {
    fn get_index(&self, coordinates: Coordiantes) -> usize{
        let (row, column) = coordinates;
        (row * self.width + column) as usize
    }

    fn next_move_inbounds(&self, coordinates: Coordiantes) -> bool{
        let (row, col) = coordinates;
        if row == 0 || row > self.height -1 || col == 0 || col > self.width - 1{
            return false
        }
        return true;
    }

    fn next_head(&self, direction: &Direction) -> Option<Coordiantes> {
        if !self.next_move_inbounds(self.get_head_coordinates()){
            return None
        }

        let ( head_row, head_column) = self.get_head_coordinates();


        match direction {
            Direction::Up => Some((head_row + 1, head_column)),
            Direction::Down => Some((head_row - 1, head_column)),
            Direction::Left => Some((head_row, head_column - 1)),
            Direction::Right => Some((head_row, head_column + 1)),
        }
    }
}


impl Universe {
    pub fn new() -> Universe {

        let width = 64;
        let height = 64;
        let head_coordinates = (5, 20);

        let head_index = head_coordinates.0 * 64 + head_coordinates.1;

        let cells = (0..width * height).map(|i| {
            if i == head_index{
                return Cell::Head
            }
            Cell::Empty
        }).collect();

        let snake = vec![head_coordinates];
        let direction = Direction::Right;

        Universe { width, height, cells, direction, snake }

    }

    pub fn get_width(&self) -> u32{
        self.width
    }

    pub fn get_height(&self) -> u32{
        self.height
    }

    pub fn get_head_coordinates(&self) -> (u32, u32){
        self.snake[0]
    }

    pub fn tick(&mut self, key_input: Option<Direction>) -> Option<Vec<ChagnedCell>> {

        if let Some(next_head) = match key_input {
            Some(direction) => self.next_head(&direction),
            None =>  self.next_head(&self.direction)
        }{

        let next_cell_index = self.get_index(next_head);
        let next_cell = &self.cells[next_cell_index];

        return match next_cell {
            Cell::Food => {
                //add cell
                let old_head = self.snake[0];
                let mut new_snake = vec![next_head];
                new_snake.extend(&self.snake.clone());
                self.snake = new_snake;

                //update grid
                let old_head_index = self.get_index(old_head);
                self.cells[old_head_index] = Cell::Tail;

                let new_head_index = self.get_index(next_head);
                self.cells[new_head_index] = Cell::Head;

                let new_head = ChagnedCell{newCell: Cell::Head, coordinate: next_head};
                let old_head = ChagnedCell{newCell: Cell::Tail, coordinate: old_head};
                Some(vec![new_head, old_head])

                //TODO: reseed food

            },
            Cell::Empty => {

                let cell_to_empty = *self.snake.last().unwrap();
                //move up everything towards head
                //this might be faster as a linked list
                for index in self.snake.len()..1{
                    self.snake[index] = self.snake[index - 1]
                }

                self.snake[0] = next_head;

                //update grid
                let next_head_index = self.get_index(next_head);
                self.cells[next_head_index] = Cell::Head;

                let chagned_head = ChagnedCell{newCell: Cell::Head, coordinate: next_head};
                let chagned_tail = ChagnedCell{newCell: Cell::Empty, coordinate: cell_to_empty};
                Some(vec![chagned_head, chagned_tail])
            },
            _ => None
        }



    }
    return None
}
}

