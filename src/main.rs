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


struct SnakeGame{
    snakeVelocityX : f64,
    snakeVelocityY : f64,
    velocity: f64,
    game_over: bool,
    dimensions: [u32; 2],
    snakeHeadPos: [f64; 2],
    snakeLength: u32,
}

impl SnakeGame{

    fn new(width: u32, height: u32) -> Self{
        SnakeGame{
            snakeVelocityX : 0.0,
            snakeVelocityY : 0.0,
            velocity : 100.0,
            game_over: false,
            dimensions: [width, height],
            snakeHeadPos: [(width as f64) * 0.5, (height as f64) * 0.5],
            snakeLength: 1,       
        }
    }

    fn is_game_over(&mut self) -> bool{
        self.game_over
    }
    
    fn on_update(&mut self, upd: &UpdateArgs){  
        // Update position of snake head
        self.snakeHeadPos[0] = self.snakeHeadPos[0] + self.snakeVelocityX*upd.dt;
        self.snakeHeadPos[1] = self.snakeHeadPos[1] + self.snakeVelocityY*upd.dt;
    }

    fn snake_move_up(&mut self){
        self.snakeVelocityX = 0.0;
        self.snakeVelocityY = -self.velocity;
    }

    fn snake_move_down(&mut self){
        self.snakeVelocityX = 0.0;
        self.snakeVelocityY = self.velocity;
    }

    fn snake_move_left(&mut self){
        self.snakeVelocityX = -self.velocity;
        self.snakeVelocityY = 0.0;
    }

    fn snake_move_right(&mut self){
        self.snakeVelocityX = self.velocity;
        self.snakeVelocityY = 0.0;
    }    

    fn on_keypress(&mut self, key : Key){
        match key {
            Key::Up => {
                println!("Pressed Up");
                self.snake_move_up();
            }
            Key::Down => {
                println!("Pressed Down");
                self.snake_move_down();
            }
            Key::Left => {
                println!("Pressed Left");
                self.snake_move_left();
            }
            Key::Right => {
                println!("Pressed Right");
                self.snake_move_right();
            }
            Key::X => {
                println!("Game over!");
                self.game_over = true;
            }
            _ => {}
        }
    }

    fn on_render(&self, args: &RenderArgs, gl: &mut GlGraphics){

        use graphics::*;
        let square = rectangle::square(self.snakeHeadPos[0], self.snakeHeadPos[1], 10.0);
        
        // draw viewport
        gl.draw(args.viewport(), |c, gl| {
            // clear the screen
            clear(color::BLACK, gl);

            // draw snakes head
            rectangle(color::WHITE, square, c.transform, gl);

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

    let mut game = SnakeGame::new(640,480);

    game.exec(&mut window,&mut gl,&mut events);
}







