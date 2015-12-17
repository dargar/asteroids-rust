extern crate cgmath;
extern crate rand;

use cgmath::Vector4;
use cgmath::Vector;
use self::rand::Rng;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub enum Size {
    Large,
    Medium,
    Small,
}

#[derive(Debug, Clone, Copy)]
pub enum Kind {
    PlayerShip,
    ProjectileFriendly,
    Asteroid(Size),
}

pub struct Entity {
    pub id: u32,
}

impl Entity {
    fn new(id: u32) -> Entity {
        Entity { id: id }
    }

    pub fn player_ship(state: &mut EntityState) -> Entity {
        let entity = Entity::new(state.next_id());
        state.add_kind(entity.id, Kind::PlayerShip);
        state.add_acceleration(entity.id, Vector4::zero());
        state.add_position(entity.id, Vector4::new(400.0, 300.0, 0.0, 1.0));
        state.add_velocity(entity.id, Vector4::zero());
        state.add_direction(entity.id, 0.0);
        state.add_model(entity.id, (1, 3));
        state.add_scale(entity.id, Vector4::new(20.0, 30.0, 0.0, 1.0));
        state.add_weapon_cooldown(entity.id, 0.0);
        entity
    }

    fn asteroid(state: &mut EntityState, size: Size, position: Option<Vector4<f32>>) -> Entity {
        let entity = Entity::new(state.next_id());
        state.add_kind(entity.id, Kind::Asteroid(size));

        let mut rng = rand::StdRng::new().expect("Could not load random number generator.");

        let p = match position {
            Some(position) => position,
            None => {
                let px = rng.next_f32() * 800.0;
                let py = rng.next_f32() * 600.0;
                Vector4::new(px, py, 0.0, 1.0)
            }
        };
        state.add_position(entity.id, p);

        let dir = rng.next_f32() * 360.0;
        state.add_direction(entity.id, dir);

        let mut acceleration: Vector4<f32> = Vector4::zero();
        acceleration.x += cgmath::sin(cgmath::deg(dir));
        acceleration.y += -cgmath::cos(cgmath::deg(dir));
        acceleration = acceleration * (100.0 + rng.next_f32() * 100.0);

        state.add_velocity(entity.id, acceleration);
        state.add_model(entity.id, (3, 10));

        let s = match size {
            Size::Large => Vector4::new(50.0, 50.0, 0.0, 1.0),
            Size::Medium => Vector4::new(25.0, 25.0, 0.0, 1.0),
            Size::Small => Vector4::new(12.5, 12.5, 0.0, 1.0),
        };
        state.add_scale(entity.id, s);

        entity
    }

    pub fn large_asteroid(state: &mut EntityState) -> Entity {
        Entity::asteroid(state, Size::Large, None)
    }

    pub fn medium_asteroid(state: &mut EntityState, position: Vector4<f32>) -> Entity {
        Entity::asteroid(state, Size::Medium, Some(position))
    }

    pub fn small_asteroid(state: &mut EntityState, position: Vector4<f32>) -> Entity {
        Entity::asteroid(state, Size::Small, Some(position))
    }

    pub fn projectile(state: &mut EntityState, pos: Vector4<f32>, dir: f32) -> Entity {
        let entity = Entity::new(state.next_id());
        state.add_kind(entity.id, Kind::ProjectileFriendly);

        state.add_position(entity.id, pos);
        state.add_direction(entity.id, dir);

        let mut acceleration: Vector4<f32> = Vector4::zero();
        acceleration.x += cgmath::sin(cgmath::deg(dir));
        acceleration.y += -cgmath::cos(cgmath::deg(dir));
        acceleration = acceleration * 500.0;

        state.add_velocity(entity.id, acceleration);
        state.add_model(entity.id, (2, 4));
        state.add_scale(entity.id, Vector4::new(5.0, 5.0, 0.0, 1.0));
        state.add_lifetime(entity.id, 0.75);

        entity
    }

    pub fn update(&self, state: &mut EntityState, t: f32) {
        let mut zero = Vector4::zero();
        let acceleration = match state.accelerations.get_mut(&self.id) {
            Some(i) => i,
            None => &mut zero,
        };
        *acceleration = *acceleration * 1000.0;
        let position = state.positions.get_mut(&self.id).unwrap();
        let velocity = state.velocities.get_mut(&self.id).unwrap();
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

        if let Some(lifetime) = state.lifetimes.get_mut(&self.id) {
            *lifetime -= t;
        }

        if let Some(weapon_cooldown) = state.weapon_cooldowns.get_mut(&self.id) {
            *weapon_cooldown -= t;
        }

        *acceleration = Vector4::zero();
    }
}

pub struct EntityState {
    entity_count: u32,
    pub kinds: HashMap<u32, Kind>,
    pub accelerations: HashMap<u32, Vector4<f32>>,
    pub positions: HashMap<u32, Vector4<f32>>,
    pub velocities: HashMap<u32, Vector4<f32>>,
    pub directions: HashMap<u32, f32>,
    pub models: HashMap<u32, (u32, u32)>,
    pub scales: HashMap<u32, Vector4<f32>>,
    pub lifetimes: HashMap<u32, f32>,
    pub weapon_cooldowns: HashMap<u32, f32>,
}

impl EntityState {
    pub fn new() -> EntityState {
        EntityState {
            entity_count: 0,
            kinds: HashMap::new(),
            accelerations: HashMap::new(),
            positions: HashMap::new(),
            velocities: HashMap::new(),
            directions: HashMap::new(),
            models: HashMap::new(),
            scales: HashMap::new(),
            lifetimes: HashMap::new(),
            weapon_cooldowns: HashMap::new(),
        }
    }

    fn next_id(&mut self) -> u32 {
        let id = self.entity_count;
        self.entity_count += 1;
        id
    }

    fn add_kind(&mut self, id: u32, kind: Kind) {
        self.kinds.insert(id, kind);
    }

    fn add_acceleration(&mut self, id: u32, acceleration: Vector4<f32>) {
        self.accelerations.insert(id, acceleration);
    }

    fn add_position(&mut self, id: u32, position: Vector4<f32>) {
        self.positions.insert(id, position);
    }

    fn add_direction(&mut self, id: u32, direction: f32) {
        self.directions.insert(id, direction);
    }

    fn add_velocity(&mut self, id: u32, velocity: Vector4<f32>) {
        self.velocities.insert(id, velocity);
    }

    fn add_model(&mut self, id: u32, model: (u32, u32)) {
        self.models.insert(id, model);
    }

    fn add_scale(&mut self, id: u32, scale: Vector4<f32>) {
        self.scales.insert(id, scale);
    }

    fn add_lifetime(&mut self, id: u32, lifetime: f32) {
        self.lifetimes.insert(id, lifetime);
    }

    fn add_weapon_cooldown(&mut self, id: u32, weapon_cooldown: f32) {
        self.weapon_cooldowns.insert(id, weapon_cooldown);
    }

    pub fn remove(&mut self, id: u32) {
        self.kinds.remove(&id);
        self.accelerations.remove(&id);
        self.positions.remove(&id);
        self.velocities.remove(&id);
        self.directions.remove(&id);
        self.models.remove(&id);
        self.scales.remove(&id);
        self.lifetimes.remove(&id);
        self.weapon_cooldowns.remove(&id);
    }
}
