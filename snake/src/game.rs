use piston_window::*;
use piston_window::types::Color;

use rand::{thread_rng, Rng};

use snake::{Direction, Snake};
use draw::{draw_block, draw_rectangle};

// Color of food(apple), using RGBA, 80% red, full opacity
const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
// Color of border, using RGBA, black, full opacity
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0];
// Color of Game Over Text, using RGBA, 90% red, 50% opacity
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];

// Equivalent of FPS (seconds)
const MOVING_PERIOD: f64 = 0.1;
// Time before restarting on gameover (seconds)
const RESTART_TIME: f64 = 1.0;

// Representation of the game
pub struct Game {
    snake: Snake,

    food_exists: bool,
    food_x: i32,
    food_y: i32,

    width: i32,
    height: i32,

    game_over: bool,
    waiting_time: f64,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            waiting_time: 0.0,
            food_exists: true,
            food_x: 6,
            food_y: 4,
            width,
            height,
            game_over: false
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        let dir = match key {
            // Allow Arrow Keys or WASD for movement

            // Arrow Keys:
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            //WASD:
            Key::W => Some(Direction::Up),
            Key::S => Some(Direction::Down),
            Key::A => Some(Direction::Left),
            Key::D => Some(Direction::Right),
            _ => None       // Any other key do nothing
        };

        // Prevent player from moving snake in opposite direction
        if dir.unwrap() == self.snake.head_direction().opposite() {
            return;
        }

        self.update_snake(dir);
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw(con, g);

        // If food has not just been eaten draw it on the screen
        if self.food_exists {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
        }

        // Draw borders
        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.height, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);

        // If game over draw game over screen
        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if !self.food_exists {
            self.add_food();
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    // Check if the snake has eaten the apple, if so append a block to the snake
    fn check_eating(&mut self) {
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.food_exists = false;
            self.snake.restore_tail();
        }
    }

    // Perform all checks to see whether or not the snake has died 
    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        // Check if snake collided with self
        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }

        // Check if snake out of bounds
        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }

    // Add food to the game board but ensure that the food is not overlapping with
    // the snake upon spawn
    fn add_food(&mut self) {
        let mut rng = thread_rng();

        let mut new_x = rng.gen_range(1, self.width - 1);
        let mut new_y = rng.gen_range(1, self.width - 1);
        while self.snake.overlap_tail(new_x, new_y) {
            new_x = rng.gen_range(1, self.width - 1);
            new_y = rng.gen_range(1, self.width - 1);
        }

        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }

    // Update snake based on current game conditions
    fn update_snake(&mut self, dir: Option<Direction>) {
        // If alive move the snake forward in cur direction
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        // else end game
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }

    // Reset game to original conditions upon restart
    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.food_exists = true;
        self.food_x = 6;
        self.food_y = 4;
        self.game_over = false;
    }
}