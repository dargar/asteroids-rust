extern crate sdl2;
mod asteroids;

fn main() {
    println!("Hello, world!");
    asteroids::update_and_render();
}
