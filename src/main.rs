extern crate sdl2;
mod asteroids;
use sdl2::event::Event;

fn main() {
    let context = sdl2::init()
        .expect("Could not initialize SDL2.");
    let mut events = context.event_pump()
        .expect("Could not create SDL2 event pump.");
    let video = context.video()
        .expect("Could not create SDL2 video subsystem.");
    let window = video.window("Asteroids", 800, 600)
        .position_centered()
        .build()
        .expect("Could not build SDL2 window.");

    'game: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} => break 'game,
                Event::KeyDown {..} => break 'game,
                _ => ()
            }
        }
        asteroids::update_and_render();
        std::thread::sleep_ms(100);
    }
}
