extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::event_loop::*;
use opengl_graphics::glyph_cache::GlyphCache;
use piston::window::WindowSettings;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use std::string;
use graphics::types::Rectangle;


/// Contains colors that are used in the game
pub mod color {
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

struct Block{
    posX : i32,
    posY : i32,
}

struct SnakeGame{
    velocity: f64,
    game_over: bool,
    dimensions: [u32; 2],
    snakeHeadPos: Block,
    snakeBody: Vec<Block>,
    direction: Direction,
    updateTime: f64,
    blockSize: f64,
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
             
        let mut centerX = ((width as f64) * 0.5) as i32;
        let mut centerY = ((height as f64) * 0.5) as i32;       
        
        SnakeGame{
            velocity : 10.0,
            game_over: false,
            dimensions: [width, height],
            snakeHeadPos: Block {posX: centerX, posY: centerY },
            snakeBody: vec![Block {posX: centerX + 1, posY: centerY},
                            Block {posX: centerX + 2, posY: centerY}],
            direction: Direction::Left,
            updateTime: 0.0,
            blockSize: 0.0,    
        }
    }

    fn is_game_over(&mut self) -> bool{
        self.game_over
    }

    fn is_collision(&mut self) -> bool{
        // is the snakehead colliding with itself?
        for block in self.snakeBody.iter(){
            if self.snakeHeadPos.posX == block.posX && self.snakeHeadPos.posY == block.posY{
                return true;
            }
        }
        // is the snakehead colliding with the border?
        if self.snakeHeadPos.posX <= 0 || self.snakeHeadPos.posX >= self.dimensions[0] as i32
        || self.snakeHeadPos.posY <= 0 || self.snakeHeadPos.posY >= self.dimensions[1] as i32{
            return true;
        }
        false
    }
    
    fn on_update(&mut self, upd: &UpdateArgs){      
        if self.is_collision(){
            println!("Game over!");
            self.game_over = true;
            return;
        }

        let (x,y) = match self.direction{
            Direction::Up =>    (0,-1),
            Direction::Down =>  (0,1),
            Direction::Right => (1,0),
            Direction::Left =>  (-1,0),
        };

        self.updateTime += upd.dt;
       
        if self.updateTime >= (1.0 / self.velocity){
            
         
            
            // Update position of snake head
            self.snakeHeadPos.posX = self.snakeHeadPos.posX + x;
            self.snakeHeadPos.posY = self.snakeHeadPos.posY + y;
            self.updateTime = 0.0;

            
        }
     

    }

    fn snake_grow(&mut self, count : u32){

    }

    fn snake_set_direction(&mut self, direction: Direction){
        
        if opposite_direction(&direction) == self.direction {
            return;
        }

        println!("Setting direction {:?}",direction);
        println!("Opposite is {:?}", opposite_direction(&direction));

        self.direction = direction;
    }

    fn on_keypress(&mut self, key : Key){
        let mut dir = match key {
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

    fn renderableRect(&self,posX : i32, posY : i32, args: &RenderArgs) -> Rectangle{
        let blockSizeX = (args.width as f64) / (self.dimensions[0] as f64);
        let blockSizeY = (args.height as f64) / (self.dimensions[1] as f64);
        let snakePosX = (posX as f64) * blockSizeX;
        let snakePosY = (posY as f64) * blockSizeY;
        let rect = graphics::rectangle::rectangle_by_corners(snakePosX - blockSizeX * 0.5, snakePosY - blockSizeY * 0.5, 
            snakePosX + blockSizeX * 0.5, snakePosY + blockSizeY * 0.5);
        rect       
    }

    fn on_render(&self, args: &RenderArgs, gl: &mut GlGraphics){

        use graphics::*;
       
        // draw viewport
        gl.draw(args.viewport(), |c, gl| {
            // clear the screen
            clear(color::BLACK, gl);

            // draw snakes head
            rectangle(color::WHITE, self.renderableRect(self.snakeHeadPos.posX,self.snakeHeadPos.posY,args), c.transform, gl);

            // draw snakes body
            for block in self.snakeBody.iter(){
                rectangle(color::WHITE, self.renderableRect(block.posX,block.posY,args), c.transform, gl);  
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







