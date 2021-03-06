extern crate cgmath;
extern crate gl;
extern crate libc;
extern crate sdl2;
extern crate time;

mod asteroids;
mod collisions;
mod entity;
mod render;

use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use std::collections::HashMap;

fn main() {
    let context = sdl2::init().expect("Could not initialize SDL2.");
    let mut events = context.event_pump()
        .expect("Could not create SDL2 event pump.");
    let video = context.video()
        .expect("Could not create SDL2 video subsystem.");
    let window = video.window("Asteroids", 800, 600)
        .opengl()
        .position_centered()
        .build()
        .expect("Could not build SDL2 window.");
    let gl_context = window.gl_create_context()
        .expect("Could not create OpenGL context.");
    window.gl_make_current(&gl_context)
        .expect("Could not make OpenGL context current.");
    gl::load_with(|s| video.gl_get_proc_address(s) as *const libc::c_void);

    let vs = include_str!("vertex_shader.glsl");
    let fs = include_str!("fragment_shader.glsl");
    let program = render::create_program(vs, fs);

    unsafe {
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        gl::Viewport(0, 0, 800, 600);
        gl::UseProgram(program);
    }

    let player_ship = vec![
        0.0, -0.5, 0.0, 1.0,
        -0.5,  0.5, 0.0, 1.0,
        0.5,  0.5, 0.0, 1.0,
    ];
    render::create_object(0, &player_ship);

    let projectile = vec![
        -0.5, -0.5, 0.0, 1.0,
        -0.5,  0.5, 0.0, 1.0,
        0.5,  0.5, 0.0, 1.0,
        0.5, -0.5, 0.0, 1.0,
    ];
    render::create_object(0, &projectile);

    let asteroid_1 = vec![
        0.1, -0.5, 0.0, 1.0,
        -0.4, -0.3, 0.0, 1.0,
        -0.2, -0.1, 0.0, 1.0,
        -0.5,  0.0, 0.0, 1.0,
        -0.4,  0.4, 0.0, 1.0,
        -0.1,  0.5, 0.0, 1.0,
        0.3,  0.2, 0.0, 1.0,
        0.2,  0.1, 0.0, 1.0,
        0.4, -0.2, 0.0, 1.0,
        0.4, -0.3, 0.0, 1.0,
    ];
    render::create_object(0, &asteroid_1);

    let mut asteroids = asteroids::Asteroids::new();

    let mut current_time = time::precise_time_ns();
    while asteroids.should_continue() {
        let previous_time = current_time;
        current_time = time::precise_time_ns();
        let delta = (current_time - previous_time) as f32 / 1_000_000_000.0;
        let mut input = HashMap::new();
        for event in events.poll_iter() {
            let c = translate_sdl2_event(event);
            let transitions = input.entry(c).or_insert(1);
            *transitions += 1;
        }
        asteroids::update_and_render(&mut asteroids, &input, delta);
        window.gl_swap_window();
    }
}

fn translate_sdl2_event(event: Event) -> char {
    match event {
        Event::Quit {..} => 'q',
        Event::KeyDown {repeat: false, scancode: Some(Scancode::Q), ..} => 'q',
        Event::KeyDown {repeat: false, scancode: Some(Scancode::W), ..} => 'w',
        Event::KeyDown {repeat: false, scancode: Some(Scancode::A), ..} => 'a',
        Event::KeyDown {repeat: false, scancode: Some(Scancode::D), ..} => 'd',
        Event::KeyDown {repeat: false, scancode: Some(Scancode::Space), ..} => ' ',
        Event::KeyUp {repeat: false, scancode: Some(Scancode::Q), ..} => 'q',
        Event::KeyUp {repeat: false, scancode: Some(Scancode::W), ..} => 'w',
        Event::KeyUp {repeat: false, scancode: Some(Scancode::A), ..} => 'a',
        Event::KeyUp {repeat: false, scancode: Some(Scancode::D), ..} => 'd',
        Event::KeyUp {repeat: false, scancode: Some(Scancode::Space), ..} => ' ',
        Event::KeyDown {repeat: false, scancode: Some(Scancode::Up), ..} => 'w',
        Event::KeyDown {repeat: false, scancode: Some(Scancode::Left), ..} => 'a',
        Event::KeyDown {repeat: false, scancode: Some(Scancode::Right), ..} => 'd',
        Event::KeyUp {repeat: false, scancode: Some(Scancode::Up), ..} => 'w',
        Event::KeyUp {repeat: false, scancode: Some(Scancode::Left), ..} => 'a',
        Event::KeyUp {repeat: false, scancode: Some(Scancode::Right), ..} => 'd',
        _ => '§',
    }
}
