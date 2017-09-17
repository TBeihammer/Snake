extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

use piston::event_loop::*;
use piston::window::WindowSettings;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use opengl_graphics::glyph_cache::GlyphCache;

mod snake;

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
    let mut glyph_cache = GlyphCache::new("../../assets/Roboto-Regular.ttf").expect("Error unwrapping fonts");
    let mut game = snake::SnakeGame::new(30,30);
    
    game.exec(&mut window,&mut gl,&mut events,&mut glyph_cache);
}







