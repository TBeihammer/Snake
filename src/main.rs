extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

use piston::event_loop::*;
use piston::window::WindowSettings;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use opengl_graphics::glyph_cache::GlyphCache;
use graphics::types::Rectangle;
use rand::Rng;

/// Contains colors that are used in the game
pub mod game_colors {
    pub const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
    pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    pub const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
    pub const LIGHTBLUE: [f32; 4] = [0.0, 1.0, 1.0, 1.0];
    pub const ORANGE: [f32; 4] = [1.0, 0.5, 0.0, 1.0];
    pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    pub const PINK: [f32; 4] = [1.0, 0.0, 1.0, 1.0];
    pub const ANGEL: [f32; 4 ] = [0.5,0.5,1.0,0.5];
    pub const GREEN: [f32; 4 ] = [0.0,0.5,0.0,1.0];
}

#[derive(Debug,Clone)]
struct Block{
    pos_x : i32,
    pos_y : i32,
}

#[derive(Clone)]
struct SnakeGame{
    velocity: f64,
    game_over: bool,
    restart: bool,
    dimensions: [u32; 2],
    snake_head: Block,
    snake_body: Vec<Block>,
    direction: Direction,
    update_time: f64,
    is_growing: bool,
    fruit : Block,
    score : u32,
}

enum Collision{
    WithFruit(Block),
    WithSnake,
    WithBorder,
    NoCollision,
}


#[derive(Debug,Clone,PartialEq,Eq)]
enum Direction{
    Up,
    Down,
    Left,
    Right
}

fn opposite_direction(dir : &Direction) -> Direction{
    match *dir {
        Direction::Up => Direction::Down,
        Direction::Down => Direction::Up,
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left,
    }
}

impl SnakeGame{
    fn new(width: u32, height: u32) -> Self{           
        let center_x = ((width as f64) * 0.5) as i32;
        let center_y = ((height as f64) * 0.5) as i32;       

        SnakeGame{
            velocity : 10.0,
            game_over: false,
            restart: false,
            dimensions: [width, height],
            snake_head: Block {pos_x: center_x, pos_y: center_y },
            snake_body: vec![Block {pos_x: center_x + 1, pos_y: center_y},
                            Block {pos_x: center_x + 2, pos_y: center_y}],
            direction: Direction::Left,
            update_time: 0.0,
            is_growing: false,
            fruit: Block{   pos_x: rand::thread_rng().gen_range(1, (width - 1) as i32), 
                            pos_y: rand::thread_rng().gen_range(1, (height - 1)  as i32) },

            score: 0,
        }
    }

    fn change_pos_fruit(&mut self){
        // simply pick a random location, might also be on the snake
        self.fruit.pos_x = rand::thread_rng().gen_range(1, (self.dimensions[0] - 1) as i32);
        self.fruit.pos_y = rand::thread_rng().gen_range(1, (self.dimensions[1] - 1) as i32);
    }

    fn is_collision(&mut self) -> Collision{
        // is the snakehead colliding with the body?
        for block in self.snake_body.iter(){
            if self.snake_head.pos_x == block.pos_x && self.snake_head.pos_y == block.pos_y{
                return Collision::WithSnake;
            }
        }
        // is the snakehead colliding with the border?
        if self.snake_head.pos_x <= 0 || self.snake_head.pos_x >= self.dimensions[0] as i32
        || self.snake_head.pos_y <= 0 || self.snake_head.pos_y >= self.dimensions[1] as i32{
            return Collision::WithBorder;
        }

        // is the snakehead colliding with the fruit?
        if self.snake_head.pos_x == self.fruit.pos_x && self.snake_head.pos_y == self.fruit.pos_y{
            return Collision::WithFruit(self.fruit.clone());
        }
        Collision::NoCollision
    }
    
    // The main update loop, process the propagated changes
    fn on_update(&mut self, upd: &UpdateArgs){            
        // Look for collision
        match self.is_collision(){          
            Collision::WithFruit(fruit) =>{
                self.snake_grow();
                self.change_pos_fruit();
            }
            Collision::NoCollision =>{

            }
            _ => {
                // WithBorder / WithSnake
                self.game_over = true;
            }
        }
        
        // We update the game logic in fixed intervalls
        self.update_time += upd.dt;
        
        if self.update_time >= (1.0 / self.velocity){
            let (x,y) = match self.direction{
                Direction::Up =>    (0,-1),
                Direction::Down =>  (0,1),
                Direction::Right => (1,0),
                Direction::Left =>  (-1,0),
            };

            if self.restart{
                let pristine = SnakeGame::new(self.dimensions[0],self.dimensions[1]);
                self.game_over = pristine.game_over;
                self.restart = pristine.restart;
                self.direction = pristine.direction;
                self.velocity = pristine.velocity;
                self.snake_body = pristine.snake_body.clone();
                self.snake_head = pristine.snake_head.clone();
                self.update_time = pristine.update_time;
                self.is_growing = pristine.is_growing;
                self.fruit = pristine.fruit.clone();
                self.score = pristine.score;
                return;
            }

            if self.game_over{
                return;
            } 

            let mut blocks = Vec::new();
            let mut oldblock = self.snake_head.clone();
            
            // Update position of snake head
            self.snake_head.pos_x = self.snake_head.pos_x + x;
            self.snake_head.pos_y = self.snake_head.pos_y + y;

            if self.is_growing{
                let block = Block{pos_x : 0, pos_y: 0};
                self.snake_body.push(block); 
                self.is_growing = false;              
            }

            for block in self.snake_body.iter_mut().rev(){
                blocks.push(oldblock);
                oldblock = block.clone();
            }

            blocks.reverse();
            self.snake_body = blocks;
            self.update_time = 0.0;                        
        }
    }

    fn snake_grow(&mut self){
        self.is_growing = true;
        self.score += 1;
    }

    fn snake_set_direction(&mut self, direction: Direction){     
        if opposite_direction(&direction) == self.direction {
            return;
        }
        self.direction = direction;
    }

    fn on_keypress(&mut self, key : Key){
        let dir = match key {
            Key::Up => {
                Direction::Up
            }
            Key::Down => {
                Direction::Down
            }
            Key::Left => {
                Direction::Left
            }
            Key::Right => {
                Direction::Right
            }
            Key::Space => {
                self.restart = true;
                return;
            }
            Key::Escape => {
                println!("Quit!");
                self.game_over = true;
                return;
            }
            _ => {
                return;
            }
        };

        self.snake_set_direction(dir);
    }

    fn renderable_rect(&self,pos_x : i32, pos_y : i32, args: &RenderArgs) -> Rectangle{
        let block_size_x = (args.width as f64) / (self.dimensions[0] as f64);
        let block_size_y = (args.height as f64) / (self.dimensions[1] as f64);
        let snake_pos_x = (pos_x as f64) * block_size_x;
        let snake_pos_y = (pos_y as f64) * block_size_y;
        let rect = graphics::rectangle::rectangle_by_corners(snake_pos_x - block_size_x * 0.5, snake_pos_y - block_size_y * 0.5, 
            snake_pos_x + block_size_x * 0.5, snake_pos_y + block_size_y * 0.5);
        rect       
    }

    fn on_game_render(&self, args: &RenderArgs, gl: &mut GlGraphics, glyph_cache: &mut GlyphCache){
        use graphics::*;

        // Only draw the "game over" screen
        if self.game_over{
            self.on_you_lost_window_render(args,gl,glyph_cache);
            return;
        }
       
        // draw viewport
        gl.draw(args.viewport(), |c, gl| {
            // clear the screen
            clear(game_colors::BLACK, gl);      
            
            // draw snakes head
            rectangle(game_colors::BLUE, self.renderable_rect(self.snake_head.pos_x,self.snake_head.pos_y,args), c.transform, gl);

            // draw fruit
            rectangle(game_colors::RED, self.renderable_rect(self.fruit.pos_x,self.fruit.pos_y,args), c.transform, gl);

            // draw borders
            let lineWidth = (args.width as f64) / (self.dimensions[0] as f64) * 0.5;
            let lineHeight = (args.height as f64) / (self.dimensions[1] as f64) * 0.5;
            line(game_colors::WHITE,
                 lineWidth,
                 [0.0, 0.0, 0.0, args.height as f64],
                 c.transform,
                 gl);
            line(game_colors::WHITE,
                 lineWidth,
                 [args.width as f64, 0.0, args.width as f64, args.height as f64],
                 c.transform,
                 gl);
            line(game_colors::WHITE,
                 lineHeight,
                 [0.0, 0.0, args.width as f64, 0.0],
                 c.transform,
                 gl);
            line(game_colors::WHITE,
                 lineHeight,
                 [0.0, args.height as f64, args.width as f64, args.height as f64],
                 c.transform,
                 gl);

            // draw snakes body
            for block in self.snake_body.iter(){
                rectangle(color::WHITE, self.renderable_rect(block.pos_x,block.pos_y,args), c.transform, gl);
            }
        });
    }

    fn on_you_lost_window_render(&self, args: &RenderArgs, gl: &mut GlGraphics, glyph_cache: &mut GlyphCache){
        use graphics::*;
       
        // draw viewport
        gl.draw(args.viewport(), |c, gl| {
            // clear the screen
            clear(game_colors::BLACK, gl);

            if self.game_over{
                // display Game over and score
                text(color::WHITE,
                    10,
                    format!("Game over! Press Space to restart, Escape to quit!")
                        .as_str(),
                    glyph_cache,
                    c.transform.trans(10.0, 10.0),
                    gl);

                text(color::WHITE,
                    10,
                    format!("Your score is {}",self.score)
                        .as_str(),
                    glyph_cache,
                    c.transform.trans(10.0, 20.0),
                    gl);
                return;
            }            
        });
    }

    pub fn exec(&mut self,
               mut window: &mut Window,
               mut gl: &mut GlGraphics,
               mut e: &mut Events,
               mut glyph_cache: &mut GlyphCache){
        while let Some(e) = e.next(window) {                       
            if let Some(r) = e.update_args() {
                self.on_update(&r);
            }
            if let Some(button) = e.press_args() {
                match button {
                    Button::Keyboard(key) => self.on_keypress(key),
                    _ => {} 
                }
            }
            if let Some(r) = e.render_args() {
                self.on_game_render(&r,gl,glyph_cache);
            }                
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Snake", (640, 480))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    let mut events = Events::new(EventSettings::new());
    events.set_ups(60);

    let mut gl = GlGraphics::new(opengl);
    let mut glyph_cache = GlyphCache::new("../../assets/Roboto-Regular.ttf").expect("Error unwraping fonts");
    let mut game = SnakeGame::new(30,30);

    game.exec(&mut window,&mut gl,&mut events,&mut glyph_cache);
}







