extern crate gl;
extern crate libc;
extern crate sdl2;

mod asteroids;

use sdl2::event::Event;
use sdl2::keyboard::Scancode;

fn main() {
    let context = sdl2::init()
        .expect("Could not initialize SDL2.");
    let mut events = context.event_pump()
        .expect("Could not create SDL2 event pump.");
    let video = context.video()
        .expect("Could not create SDL2 video subsystem.");
    let window = video.window("Asteroids", 800, 600)
        .opengl()
        .position_centered()
        .build()
        .expect("Could not build SDL2 window.");

    gl::load_with(|s| video.gl_get_proc_address(s) as *const libc::c_void);

    let mut asteroids = asteroids::Asteroids::new();

    while asteroids.should_continue() {
        let input = events
            .poll_iter()
            .map(|e| translate_sdl2_event(e))
            .collect::<Vec<char>>();
        asteroids::update_and_render(&mut asteroids, &input);
        window.gl_swap_window();
        std::thread::sleep_ms(100);
    }
}

fn translate_sdl2_event(event: Event) -> char {
    match event {
        Event::Quit {..} => 'q',
        Event::KeyDown {scancode: Some(Scancode::Q), ..} => 'q',
        _ => ' ',
    }
}
