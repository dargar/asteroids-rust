extern crate cgmath;
extern crate gl;

use cgmath::Matrix;
use super::render;

pub struct Asteroids {
    should_continue: bool,
    vao: u32,
}

impl Asteroids {
    pub fn new() -> Asteroids {
        Asteroids {
            should_continue: true,
            vao: 0,
        }
    }

    pub fn should_continue(&self) -> bool {
        self.should_continue
    }
}

pub fn update_and_render(asteroids: &mut Asteroids, input: &[char]) {
    if asteroids.vao == 0 {
        asteroids.vao = render::create_object(0);
    }

    if input.iter().any(|&i| i == 'q') {
        asteroids.should_continue = false;
        return
    }

    let projection = cgmath::ortho(0.0f32, 800.0, 600.0, 0.0, -1.0, 1.0);
    let x: [f32; 16] = *projection.as_ref();

    let m = cgmath::Matrix4::new(
        100.0,   0.0, 0.0, 0.0,
          0.0, 100.0, 0.0, 0.0,
          0.0,   0.0, 1.0, 0.0,
        400.0, 300.0, 0.0, 1.0,
    );
    let y: [f32; 16] = *m.as_ref();

    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);
        gl::UniformMatrix4fv(1, 1, gl::FALSE, x.as_ptr());
        gl::UniformMatrix4fv(3, 1, gl::FALSE, y.as_ptr());
        gl::DrawArrays(gl::TRIANGLES, 0, 3);
    }
}
