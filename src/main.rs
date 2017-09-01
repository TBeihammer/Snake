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

struct SnakeGame{
    velocity: f64,
    game_over: bool,
    dimensions: [u32; 2],
    snake_head: Block,
    snake_body: Vec<Block>,
    direction: Direction,
    update_time: f64,
    is_growing: bool,
    fruit : Block,
}

enum Collision{
    With_Fruit(Block),
    With_Snake,
    With_Border,
    No_Collision,
}


#[derive(Debug,Clone,PartialEq,Eq)]
enum Direction{
    Up,
    Down,
    Left,
    Right
}

    fn opposite_direction(dir : &Direction) -> Direction {
        match *dir {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

impl SnakeGame{
    fn new(width: u32, height: u32) -> Self{
             
        let centerX = ((width as f64) * 0.5) as i32;
        let centerY = ((height as f64) * 0.5) as i32;       

        SnakeGame{
            velocity : 10.0,
            game_over: false,
            dimensions: [width, height],
            snake_head: Block {pos_x: centerX, pos_y: centerY },
            snake_body: vec![Block {pos_x: centerX + 1, pos_y: centerY},
                            Block {pos_x: centerX + 2, pos_y: centerY}],
            direction: Direction::Left,
            update_time: 0.0,
            is_growing: false,
            fruit: Block{   pos_x: rand::thread_rng().gen_range(1, (width - 1) as i32), 
                            pos_y: rand::thread_rng().gen_range(1, (height - 1)  as i32) },
        }
    }

    fn is_game_over(&mut self) -> bool{
        self.game_over
    }

    fn change_pos_fruit(&mut self){
        self.fruit.pos_x = rand::thread_rng().gen_range(1, (self.dimensions[0] - 1) as i32);
        self.fruit.pos_y = rand::thread_rng().gen_range(1, (self.dimensions[1] - 1) as i32);
    }

    fn is_collision(&mut self) -> Collision{
        // is the snakehead colliding with itself?
        for block in self.snake_body.iter(){
            if self.snake_head.pos_x == block.pos_x && self.snake_head.pos_y == block.pos_y{
                return Collision::With_Snake;
            }
        }
        // is the snakehead colliding with the border?
        if self.snake_head.pos_x <= 0 || self.snake_head.pos_x >= self.dimensions[0] as i32
        || self.snake_head.pos_y <= 0 || self.snake_head.pos_y >= self.dimensions[1] as i32{
            return Collision::With_Border;
        }

        // is the snakehead colliding with the fruit?
        if self.snake_head.pos_x == self.fruit.pos_x && self.snake_head.pos_y == self.fruit.pos_y{
            return Collision::With_Fruit(self.fruit.clone());
        }
        Collision::No_Collision
    }
    
    fn on_update(&mut self, upd: &UpdateArgs){      
        
        match self.is_collision(){          
                Collision::With_Fruit(fruit) =>{
                    self.snake_grow();
                    self.change_pos_fruit();
                }
                Collision::No_Collision =>{
                }         
                _ =>{
                    println!("Game over!");
                    self.game_over = true;
                    return;
                }
            }
        
        self.update_time += upd.dt;
        
        if self.update_time >= (1.0 / self.velocity){
            let (x,y) = match self.direction{
                Direction::Up =>    (0,-1),
                Direction::Down =>  (0,1),
                Direction::Right => (1,0),
                Direction::Left =>  (-1,0),
            };

            let mut blocks = Vec::new();
            let mut oldblock = self.snake_head.clone();
            
            // Update position of snake head
            self.snake_head.pos_x = self.snake_head.pos_x + x;
            self.snake_head.pos_y = self.snake_head.pos_y + y;

            if (self.is_growing){
                let mut block = Block{pos_x : 0, pos_y: 0};
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
            Key::X => {
                println!("Game over!");
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

    fn on_render(&self, args: &RenderArgs, gl: &mut GlGraphics){

        use graphics::*;
       
        // draw viewport
        gl.draw(args.viewport(), |c, gl| {
            // clear the screen
            clear(game_colors::BLACK, gl);

            // draw snakes head
            rectangle(game_colors::BLUE, self.renderable_rect(self.snake_head.pos_x,self.snake_head.pos_y,args), c.transform, gl);

            // draw fruit
            rectangle(game_colors::RED, self.renderable_rect(self.fruit.pos_x,self.fruit.pos_y,args), c.transform, gl);

            // draw snakes body
            for block in self.snake_body.iter(){
                rectangle(color::WHITE, self.renderable_rect(block.pos_x,block.pos_y,args), c.transform, gl);  
            }
        });
    }

    pub fn exec(&mut self,
               mut window: &mut Window,
               mut gl: &mut GlGraphics,
               mut e: &mut Events){

        self.game_over = false;
        while let Some(e) = e.next(window) {
            if self.is_game_over(){
                break;
            }                            
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
                self.on_render(&r,gl);
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

    let mut game = SnakeGame::new(30,30);

    game.exec(&mut window,&mut gl,&mut events);
}







