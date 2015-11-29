extern crate cgmath;
extern crate gl;

use cgmath::Matrix4;
use cgmath::Matrix;
use cgmath::SquareMatrix;
use cgmath::Vector4;
use cgmath::Vector;
use super::entity::Entity;
use super::render;

pub struct Asteroids {
    should_continue: bool,
    vao: u32,
    projection: Matrix4<f32>,
    entity: Entity,
}

impl Asteroids {
    pub fn new() -> Asteroids {
        Asteroids {
            should_continue: true,
            vao: 0,
            projection: cgmath::ortho(0.0, 800.0, 600.0, 0.0, -1.0, 1.0),
            entity: Entity::new(0, Vector4::new(400.0, 300.0, 0.0, 1.0)),
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

    let mut a: Vector4<f32> = Vector4::zero();
    for &event in input {
        match event {
            'q' => {
                asteroids.should_continue = false;
                return
            }
            'w' => {
                a.x += cgmath::sin(cgmath::deg(asteroids.entity.direction));
                a.y += -cgmath::cos(cgmath::deg(asteroids.entity.direction));
            },
            'a' => asteroids.entity.direction += -4.0,
            'd' => asteroids.entity.direction += 4.0,
            _ => (),
        }
    }

    asteroids.entity.acceleration = a;
    asteroids.entity.update(1.0 / 60.0);

    let mut model = Matrix4::one();

    let mut translation = Matrix4::one();
    translation.replace_col(3, asteroids.entity.position);
    model = model.mul_m(&translation);
    
    let mut rotation_z = Matrix4::one();
    let theta = asteroids.entity.direction;
    rotation_z[0][0] = cgmath::cos(cgmath::deg(theta));
    rotation_z[0][1] = cgmath::sin(cgmath::deg(theta));
    rotation_z[1][0] = -cgmath::sin(cgmath::deg(theta));
    rotation_z[1][1] = cgmath::cos(cgmath::deg(theta));
    model = model.mul_m(&rotation_z);

    let scaling = Matrix4::from_diagonal(cgmath::Vector4::new(25.0f32, 50.0f32, 0.0f32, 1.0f32));
    model = model.mul_m(&scaling);

    let mvp = asteroids.projection.mul_m(&model);
    let mvp_array: [f32; 16] = *mvp.as_ref();

    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);
        gl::UniformMatrix4fv(1, 1, gl::FALSE, mvp_array.as_ptr());
        gl::DrawArrays(gl::LINE_LOOP, 0, 3);
    }
}
