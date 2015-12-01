extern crate cgmath;
extern crate gl;

use cgmath::Matrix4;
use cgmath::Matrix;
use cgmath::SquareMatrix;
use cgmath::Vector4;
use cgmath::Vector;
use super::entity::Component;
use super::entity::Entity;
use super::entity::EntityState;
use super::render;

pub struct Asteroids {
    should_continue: bool,
    vao: u32,
    projection: Matrix4<f32>,
    entities: Vec<Entity>,
    state: EntityState,
}

impl Asteroids {
    pub fn new() -> Asteroids {
        let entity_state = EntityState::new();
        Asteroids {
            should_continue: true,
            vao: 0,
            projection: cgmath::ortho(0.0, 800.0, 600.0, 0.0, -1.0, 1.0),
            entities: Vec::new(),
            state: entity_state,
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

    if asteroids.entities.is_empty() {
        asteroids.entities.push(Entity::player_ship(&mut asteroids.state));
        asteroids.entities.push(Entity::asteroid(&mut asteroids.state));
        asteroids.entities.push(Entity::asteroid(&mut asteroids.state));
        asteroids.entities.push(Entity::asteroid(&mut asteroids.state));
    }

    {
        let ref mut direction = asteroids.state.directions[*asteroids.entities[0].components.get(&Component::Direction).unwrap()];
        let ref mut acceleration = asteroids.state.accelerations[*asteroids.entities[0].components.get(&Component::Acceleration).unwrap()];
        for &event in input {
            match event {
                'w' => {
                    acceleration.x += cgmath::sin(cgmath::deg(*direction));
                    acceleration.y += -cgmath::cos(cgmath::deg(*direction));
                },
                'a' => *direction += -4.0,
                'd' => *direction += 4.0,
                'q' => {
                    asteroids.should_continue = false;
                    return
                }
                _ => (),
            }
        }
    }

    for entity in &asteroids.entities {
        entity.update(&mut asteroids.state, 1.0 / 60.0);
    }

    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }

    for entity in &asteroids.entities {
        let mut model = Matrix4::one();

        let mut translation = Matrix4::one();
        let position = asteroids.state.positions[*entity.components.get(&Component::Position).unwrap()];
        translation.replace_col(3, position);
        model = model.mul_m(&translation);

        let mut rotation_z = Matrix4::one();
        let theta = asteroids.state.directions[*entity.components.get(&Component::Direction).unwrap()];
        rotation_z[0][0] = cgmath::cos(cgmath::deg(theta));
        rotation_z[0][1] = cgmath::sin(cgmath::deg(theta));
        rotation_z[1][0] = -cgmath::sin(cgmath::deg(theta));
        rotation_z[1][1] = cgmath::cos(cgmath::deg(theta));
        model = model.mul_m(&rotation_z);

        let scaling = Matrix4::from_diagonal(Vector4::new(25.0f32, 50.0f32, 0.0f32, 1.0f32));
        model = model.mul_m(&scaling);

        let mvp = asteroids.projection.mul_m(&model);
        let mvp_array: [f32; 16] = *mvp.as_ref();

        unsafe {
            gl::UniformMatrix4fv(1, 1, gl::FALSE, mvp_array.as_ptr());
            gl::DrawArrays(gl::LINE_LOOP, 0, 3);
        }
    }
}
