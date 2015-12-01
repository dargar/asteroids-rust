extern crate cgmath;
extern crate rand;

use cgmath::Vector4;
use cgmath::Vector;
use self::rand::Rng;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash)]
pub enum Component {
    Acceleration,
    Position,
    Velocity,
    Direction,
}

pub struct Entity {
    pub id: u32,
    pub components: HashMap<Component, usize>,
}

impl Entity {
    fn new(id: u32) -> Entity {
        Entity {
            id: id,
            components: HashMap::new(),
        }
    }

    pub fn player_ship(state: &mut EntityState) -> Entity {
        let mut entity = Entity::new(state.next_id());

        let acceleration = state.add_acceleration(Vector4::zero());
        entity.components.insert(Component::Acceleration, acceleration);

        let position = state.add_position(Vector4::new(400.0, 300.0, 0.0, 1.0));
        entity.components.insert(Component::Position, position);

        let velocity = state.add_velocity(Vector4::zero());
        entity.components.insert(Component::Velocity, velocity);

        let direction = state.add_direction(0.0);
        entity.components.insert(Component::Direction, direction);

        entity
    }

    pub fn asteroid(state: &mut EntityState) -> Entity {
        let mut entity = Entity::new(state.next_id());

        let mut rng = rand::StdRng::new()
            .expect("Could not load random number generator.");

        let px = rng.next_f32() * 800.0;
        let py = rng.next_f32() * 600.0;
        let position = state.add_position(Vector4::new(px, py, 0.0, 1.0));
        entity.components.insert(Component::Position, position);

        let dir = rng.next_f32() * 360.0;
        let direction = state.add_direction(dir);
        entity.components.insert(Component::Direction, direction);

        let mut acceleration: Vector4<f32> = Vector4::zero();
        acceleration.x += cgmath::sin(cgmath::deg(dir));
        acceleration.y += -cgmath::cos(cgmath::deg(dir));
        acceleration = acceleration * 150.0;
        
        let velocity = state.add_velocity(acceleration);
        entity.components.insert(Component::Velocity, velocity);

        entity
    }

    pub fn update(&self, state: &mut EntityState, t: f32) {
        let mut zero = Vector4::zero();
        let acceleration = match self.components.get(&Component::Acceleration) {
            Some(&i) => &mut state.accelerations[i],
            None => &mut zero,
        };
        *acceleration = *acceleration * 1000.0;
        let ref mut position = state.positions[*self.components.get(&Component::Position).unwrap()];
        let ref mut velocity = state.velocities[*self.components.get(&Component::Velocity).unwrap()];
        *position = *acceleration * t * t * 0.5f32 + *velocity * t + *position;
        *velocity = *acceleration * t + *velocity;

        if position.x < 0.0 {
            position.x = 800.0;
        } else if position.x > 800.0 {
            position.x = 0.0;
        }

        if position.y < 0.0 {
            position.y = 600.0;
        } else if position.y > 600.0 {
            position.y = 0.0;
        }

        *acceleration = Vector4::zero();
    }
}

// FIXME: Indices might be wrong after removing one or more values
pub struct EntityState {
    entity_count: u32,
    pub accelerations: Vec<Vector4<f32>>,
    pub positions: Vec<Vector4<f32>>,
    pub velocities: Vec<Vector4<f32>>,
    pub directions: Vec<f32>,
}

impl EntityState {
    pub fn new() -> EntityState {
        EntityState {
            entity_count: 0,
            accelerations: Vec::new(),
            positions: Vec::new(),
            velocities: Vec::new(),
            directions: Vec::new(),
        }
    }

    fn next_id(&mut self) -> u32 {
        let id = self.entity_count;
        self.entity_count += 1;
        id
    }

    fn add_acceleration(&mut self, acceleration: Vector4<f32>) -> usize {
        let index = self.accelerations.len();
        self.accelerations.push(acceleration);
        index
    }

    fn add_position(&mut self, position: Vector4<f32>) -> usize {
        let index = self.positions.len();
        self.positions.push(position);
        index
    }

    fn add_direction(&mut self, direction: f32) -> usize {
        let index = self.directions.len();
        self.directions.push(direction);
        index
    }

    fn add_velocity(&mut self, velocity: Vector4<f32>) -> usize {
        let index = self.velocities.len();
        self.velocities.push(velocity);
        index
    }
}
