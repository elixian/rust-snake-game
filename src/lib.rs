use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;
use rand::prelude::*;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}
#[derive(Clone, Copy)]
pub struct SnakeCell(u32);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    fn new(spawn_index: u32, size: u32) -> Snake {
        // let mut body = vec!();

        // for i in 0..size{
        //     body.push(SnakeCell(spawn_index - i))
        // }

        Snake {
            body: Snake::init_body(spawn_index, size),
            direction: Direction::Down,
        }
    }

    fn init_body(idx: u32, size: u32) -> Vec<SnakeCell> {
        let mut ve_snake: Vec<SnakeCell> = vec![];
        for i in 0..size {
            ve_snake.push(SnakeCell(idx - i));
        }
        ve_snake
    }
}
#[wasm_bindgen]
pub struct World {
    width: u32,
    size: u32,
    snake: Snake,
    next_cell: Option<SnakeCell>,
    reward_cell : usize
}

#[wasm_bindgen]
impl World {
    pub fn new(width: u32, spawn_idx: u32) -> World {
        World {
            width,
            size: width * width,
            snake: Snake::new(spawn_idx, 3),
            next_cell: None,
            reward_cell: 10,
        }
    }
    //private
    fn random_cell(&self)-> u32{
        let mut  rng = rand::thread_rng();
        let rand:u32 = rng.gen();
        rand % self.size
    }
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn snake_head_idx(&self) -> u32 {
        self.snake.body[0].0
    }

    pub fn reward_cells(&self)-> usize {
        self.reward_cell
    }

    pub fn change_dir(&mut self, direction: Direction) {
        let next_cell =  self.gen_next_snake_cell(&direction);

        if next_cell.0 == self.snake.body[1].0 {return;}
        self.next_cell = Some(next_cell);
        self.snake.direction = direction
    }

    pub fn snake_cells(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }

    pub fn snake_len(&self) -> usize {
        self.snake.body.len()
    }

    pub fn step(&mut self) {
        let cells_copy = self.snake.body.clone();
        let  len_snake: usize =  self.snake_len();

        match self.next_cell {
            Some(cell) =>{
                self.snake.body[0] = cell;
                self.next_cell  = None;
            },
            None =>{
                self.snake.body[0] = self.gen_next_snake_cell(&self.snake.direction);
            }
        }
        
        //get current cell
        for i in 1..len_snake{
            self.snake.body[i] = SnakeCell(cells_copy[i-1].0);
        }
    }

    fn gen_next_snake_cell(&self, direction: &Direction ) -> SnakeCell {
        let snake_idx = self.snake_head_idx();
        let row = snake_idx / self.width;
        return match direction {
            Direction::Right => {
                let treshold = (row + 1) * self.width;
                if snake_idx + 1 == treshold {
                    SnakeCell(treshold - self.width)
                } else {
                    SnakeCell(snake_idx + 1)
                }
            } //((col + 1) % self.width, row),
            Direction::Left => {
                let treshold = row * self.width ;
                if snake_idx  == treshold {
                    SnakeCell(treshold + (self.width-1))
                } else {
                    SnakeCell(snake_idx - 1)
                }
            }
            Direction::Down => { //TODO Not finish
                let treshold = snake_idx - (row * self.width);
                if row + 1  == self.width {
                    SnakeCell(treshold)
                } else {
                    SnakeCell(snake_idx + self.width)
                } 
            },
            Direction::Up => {
                let treshold = snake_idx - (row * self.width);
                if treshold  == snake_idx {
                    SnakeCell((self.size - self.width) + treshold)
                } else {
                    SnakeCell(snake_idx - self.width)
                } 
            },
        };
    }

}
