extern crate cgmath;
extern crate gl;
extern crate time;

use cgmath::Matrix4;
use cgmath::Matrix;
use cgmath::SquareMatrix;
use cgmath::Vector;
use std::collections::HashMap;
use std::collections::HashSet;
use super::collisions;
use super::entity::Entity;
use super::entity::EntityState;
use super::entity::Kind;
use super::entity::Size;

enum InputStatus {
    Up,
    Down,
}

pub struct Asteroids {
    should_continue: bool,
    stage: u32,
    // TODO: Should some of these be in entity state instead?
    score: u32,
    lives: u32,
    live_up: u32,
    invulnerability_time: f32,
    projection: Matrix4<f32>,
    entities: Vec<Entity>,
    state: EntityState,
    input: HashMap<char, InputStatus>,
}

impl Asteroids {
    pub fn new() -> Asteroids {
        let entity_state = EntityState::new();
        Asteroids {
            should_continue: true,
            stage: 1,
            score: 0,
            lives: 3,
            live_up: 0,
            invulnerability_time: 0.0,
            projection: cgmath::ortho(0.0, 800.0, 600.0, 0.0, -1.0, 1.0),
            entities: Vec::new(),
            state: entity_state,
            input: HashMap::new(),
        }
    }

    pub fn should_continue(&self) -> bool {
        self.should_continue
    }
}

pub fn update_and_render(asteroids: &mut Asteroids, input: &HashMap<char, u32>, dt: f32) {
    if asteroids.entities.is_empty() {
        asteroids.entities.push(Entity::player_ship(&mut asteroids.state));
    } else if asteroids.entities.len() == 1 {
        for _ in 1..asteroids.stage {
            asteroids.entities.push(Entity::large_asteroid(&mut asteroids.state));
            asteroids.entities.push(Entity::large_asteroid(&mut asteroids.state));
            asteroids.entities.push(Entity::large_asteroid(&mut asteroids.state));
        }
        asteroids.stage += 1;
    }

    for (&event, &transition_count) in input {
        if transition_count % 2 == 0 {
            let status = asteroids.input.entry(event).or_insert(InputStatus::Up);
            match *status {
                InputStatus::Up => *status = InputStatus::Down,
                InputStatus::Down => *status = InputStatus::Up,
            }
        }
    }

    let mut projectiles = 0;
    {
        let entity_id = asteroids.entities[0].id;
        let direction = asteroids.state.directions.get_mut(&entity_id).unwrap();
        let acceleration = asteroids.state.accelerations.get_mut(&entity_id).unwrap();
        let weapon_cooldown = asteroids.state.weapon_cooldowns.get(&entity_id).unwrap();
        for ins in &asteroids.input {
            match ins {
                (&'w', &InputStatus::Down) => {
                    acceleration.x += cgmath::sin(cgmath::deg(*direction));
                    acceleration.y += -cgmath::cos(cgmath::deg(*direction));
                }
                (&'a', &InputStatus::Down) => *direction += -5.0,
                (&'d', &InputStatus::Down) => *direction += 5.0,
                (&' ', &InputStatus::Down) => {
                    if *weapon_cooldown <= 0.0 {
                        projectiles += 1;
                    }
                }
                (&'q', &InputStatus::Down) => {
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
        let weapon_cooldown = asteroids.state.weapon_cooldowns.get_mut(&entity_id).unwrap();
        *weapon_cooldown = 0.2;
    }

    asteroids.invulnerability_time -= dt;

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
    let collisions = collisions::find_collisions(&asteroids.state, &asteroids.entities);

    // Collect destroyed entities
    let mut destroyed = HashSet::new();
    for ((a, kind_a), (b, kind_b)) in collisions {
        match (kind_a, kind_b) {
            (Kind::PlayerShip, Kind::Asteroid(_)) => {
                if asteroids.invulnerability_time >= 0.0 {
                    ()
                } else if asteroids.lives <= 1 {
                    destroyed.insert(a);
                } else {
                    asteroids.lives -= 1;
                    asteroids.invulnerability_time = 1.0;
                }
            }
            (Kind::Asteroid(_), Kind::PlayerShip) => {
                if asteroids.invulnerability_time >= 0.0 {
                    ()
                } else if asteroids.lives <= 1 {
                    destroyed.insert(b);
                } else {
                    asteroids.lives -= 1;
                    asteroids.invulnerability_time = 1.0;
                }
            }
            (Kind::Asteroid(s), Kind::ProjectileFriendly) |
            (Kind::ProjectileFriendly, Kind::Asteroid(s)) => {
                destroyed.insert(a);
                destroyed.insert(b);
                let points = match s {
                    Size::Large => 10,
                    Size::Medium => 25,
                    Size::Small => 50,
                };
                // TODO: Score gets counted twice! Once for (a, b) and one for (b, a)
                asteroids.score += points;
                println!("Score: {}", asteroids.score);
                asteroids.live_up += points;
                // TODO: Verify that this is correct
                if asteroids.live_up >= 2000 {
                    asteroids.lives += 1;
                    asteroids.live_up = asteroids.live_up % 2000;
                }
            }
            _ => (),
        }
    }

    // Remove destroyed entities
    asteroids.entities.retain(|e| !destroyed.contains(&e.id));
    for d in destroyed {
        match *asteroids.state.kinds.get(&d).unwrap() {
            Kind::Asteroid(Size::Large) => {
                let position = *asteroids.state.positions.get(&d).unwrap();
                asteroids.entities.push(Entity::medium_asteroid(&mut asteroids.state, position));
                asteroids.entities.push(Entity::medium_asteroid(&mut asteroids.state, position));
            }
            Kind::Asteroid(Size::Medium) => {
                let position = *asteroids.state.positions.get(&d).unwrap();
                asteroids.entities.push(Entity::small_asteroid(&mut asteroids.state, position));
                asteroids.entities.push(Entity::small_asteroid(&mut asteroids.state, position));
            }
            Kind::PlayerShip => asteroids.should_continue = false,
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

    for life in 0..asteroids.lives {
        let mut model = Matrix4::one();

        let mut translation = Matrix4::one();
        let position = cgmath::vec4(20.0 + 25.0 * life as f32, 20.0, 0.0, 1.0);
        translation.replace_col(3, position);
        model = model.mul_m(&translation);

        let rotation_z = Matrix4::one();
        model = model.mul_m(&rotation_z);

        let scale = cgmath::vec4(20.0, 30.0, 0.0, 1.0);
        let scaling = Matrix4::from_diagonal(scale);
        model = model.mul_m(&scaling);

        let mvp = asteroids.projection.mul_m(&model);
        let mvp_array: [f32; 16] = *mvp.as_ref();

        unsafe {
            let (vao, vertices) = *asteroids.state.models.get(&asteroids.entities[0].id).unwrap();
            gl::BindVertexArray(vao);
            gl::UniformMatrix4fv(1, 1, gl::FALSE, mvp_array.as_ptr());
            gl::DrawArrays(gl::LINE_LOOP, 0, vertices as i32);
        }
    }
}
