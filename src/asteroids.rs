extern crate cgmath;
extern crate gl;

use cgmath::Matrix4;
use cgmath::Matrix;
use cgmath::SquareMatrix;
use cgmath::Vector;
use std::collections::HashSet;
use super::entity::Entity;
use super::entity::EntityState;
use super::entity::Kind;
use super::entity::Size;

pub struct Asteroids {
    should_continue: bool,
    projection: Matrix4<f32>,
    entities: Vec<Entity>,
    state: EntityState,
}

impl Asteroids {
    pub fn new() -> Asteroids {
        let entity_state = EntityState::new();
        Asteroids {
            should_continue: true,
            projection: cgmath::ortho(0.0, 800.0, 600.0, 0.0, -1.0, 1.0),
            entities: Vec::new(),
            state: entity_state,
        }
    }

    pub fn should_continue(&self) -> bool {
        self.should_continue
    }
}

pub fn update_and_render(asteroids: &mut Asteroids, input: &[char], dt: f32) {
    if asteroids.entities.is_empty() {
        asteroids.entities.push(Entity::player_ship(&mut asteroids.state));
        asteroids.entities.push(Entity::large_asteroid(&mut asteroids.state));
        asteroids.entities.push(Entity::large_asteroid(&mut asteroids.state));
        asteroids.entities.push(Entity::large_asteroid(&mut asteroids.state));
    }

    let mut projectiles = 0;
    {
        let entity_id = asteroids.entities[0].id;
        let direction = asteroids.state.directions.get_mut(&entity_id).unwrap();
        let acceleration = asteroids.state.accelerations.get_mut(&entity_id).unwrap();
        for &event in input {
            match event {
                'w' => {
                    acceleration.x += cgmath::sin(cgmath::deg(*direction));
                    acceleration.y += -cgmath::cos(cgmath::deg(*direction));
                }
                'a' => *direction += -4.0,
                'd' => *direction += 4.0,
                ' ' => projectiles += 1,
                'q' => {
                    asteroids.should_continue = false;
                    return;
                }
                _ => (),
            }
        }
    }

    if projectiles > 0 {
        let entity_id = asteroids.entities[0].id;
        let position = asteroids.state.positions.get(&entity_id).unwrap().clone();
        let direction = asteroids.state.directions.get(&entity_id).unwrap().clone();
        asteroids.entities.push(Entity::projectile(&mut asteroids.state, position, direction));
    }

    for entity in &asteroids.entities {
        entity.update(&mut asteroids.state, dt);
    }

    // Remove entities whose lifetime has run out
    let dead = asteroids.state
                        .lifetimes
                        .iter()
                        .filter(|&(_, lifetime)| *lifetime <= 0.0)
                        .map(|(id, _)| *id)
                        .collect::<Vec<_>>();
    asteroids.entities.retain(|e| !dead.contains(&e.id));
    for id in dead {
        asteroids.state.remove(id);
    }

    // Collect all collisions
    let mut collisions: Vec<((u32, Kind), (u32, Kind))> = Vec::new();
    for a in &asteroids.entities {
        let a_position = *asteroids.state.positions.get(&a.id).unwrap();
        let a_scale = *asteroids.state.scales.get(&a.id).unwrap();
        let a_min = a_position - a_scale / 2.0;
        let a_max = a_position + a_scale / 2.0;
        for b in &asteroids.entities {
            let b_position = *asteroids.state.positions.get(&b.id).unwrap();
            let b_scale = *asteroids.state.scales.get(&b.id).unwrap();
            let b_min = b_position - b_scale / 2.0;
            let b_max = b_position + b_scale / 2.0;
            if b_min.x > a_min.x && b_min.y > a_min.y && b_min.x < a_max.x && b_min.y < a_max.y {
                let kind_a = asteroids.state.kinds.get(&a.id).unwrap();
                let kind_b = asteroids.state.kinds.get(&b.id).unwrap();
                collisions.push(((a.id, (*kind_a).clone()), (b.id, (*kind_b).clone())));
            } else if b_max.x > a_min.x && b_max.y > a_min.y && b_max.x < a_max.x && b_max.y < a_max.y {
                let kind_a = asteroids.state.kinds.get(&a.id).unwrap();
                let kind_b = asteroids.state.kinds.get(&b.id).unwrap();
                collisions.push(((a.id, (*kind_a).clone()), (b.id, (*kind_b).clone())));
            }
        }
    }

    // Collect destroyed entities
    let mut destroyed = HashSet::new();
    for ((a, kind_a), (b, kind_b)) in collisions {
        match (kind_a, kind_b) {
            (Kind::PlayerShip, Kind::Asteroid(_)) => {
                destroyed.insert(a);
            }
            (Kind::Asteroid(_), Kind::PlayerShip) => {
                destroyed.insert(b);
            }
            (Kind::Asteroid(_), Kind::ProjectileFriendly) |
            (Kind::ProjectileFriendly, Kind::Asteroid(_)) => {
                destroyed.insert(a);
                destroyed.insert(b);
            }
            _ => (),
        }
    }

    // Remove destroyed entities
    asteroids.entities.retain(|e| !destroyed.contains(&e.id));
    for d in destroyed {
        match *asteroids.state.kinds.get(&d).unwrap() {
            Kind::Asteroid(Size::Large) => {
                asteroids.entities.push(Entity::medium_asteroid(&mut asteroids.state));
                asteroids.entities.push(Entity::medium_asteroid(&mut asteroids.state));
            }
            Kind::Asteroid(Size::Medium) => {
                asteroids.entities.push(Entity::small_asteroid(&mut asteroids.state));
                asteroids.entities.push(Entity::small_asteroid(&mut asteroids.state));
            }
            _ => (),
        }
        asteroids.state.remove(d);
    }

    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }

    // Draw entities
    for entity in &asteroids.entities {
        let mut model = Matrix4::one();

        let mut translation = Matrix4::one();
        let position = asteroids.state.positions.get(&entity.id).unwrap();
        translation.replace_col(3, *position);
        model = model.mul_m(&translation);

        let mut rotation_z = Matrix4::one();
        let theta = *asteroids.state.directions.get(&entity.id).unwrap();
        rotation_z[0][0] = cgmath::cos(cgmath::deg(theta));
        rotation_z[0][1] = cgmath::sin(cgmath::deg(theta));
        rotation_z[1][0] = -cgmath::sin(cgmath::deg(theta));
        rotation_z[1][1] = cgmath::cos(cgmath::deg(theta));
        model = model.mul_m(&rotation_z);

        let scale = *asteroids.state.scales.get(&entity.id).unwrap();
        let scaling = Matrix4::from_diagonal(scale);
        model = model.mul_m(&scaling);

        let mvp = asteroids.projection.mul_m(&model);
        let mvp_array: [f32; 16] = *mvp.as_ref();

        unsafe {
            let (vao, vertices) = *asteroids.state.models.get(&entity.id).unwrap();
            gl::BindVertexArray(vao);
            gl::UniformMatrix4fv(1, 1, gl::FALSE, mvp_array.as_ptr());
            gl::DrawArrays(gl::LINE_LOOP, 0, vertices as i32);
        }
    }
}
