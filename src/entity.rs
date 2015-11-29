extern crate cgmath;

use cgmath::Vector4;

pub struct Entity {
    pub id: u32,
    pub position: Vector4<f32>,
    pub velocity: Vector4<f32>,
    pub direction: f32,
}

impl Entity {
    pub fn new(id: u32, position: Vector4<f32>) -> Entity {
        Entity {
            id: id,
            position: position,
            velocity: Vector4::new(0.0, 0.0, 0.0, 0.0),
            direction: 0.0,
        }
    }

    pub fn update(&mut self, t: f32, mut a: Vector4<f32>) {
        // position, velocity, acceleration
        // p' = 1/2at^2 + vt + p
        // v' = at + v
        // a  = a
        a = a * 1000.0;
        self.position = a * t * t * 0.5f32 + self.velocity * t + self.position;
        self.velocity = a * t + self.velocity;

        if self.position.x < 0.0 {
            self.position.x = 800.0;
        } else if self.position.x > 800.0 {
            self.position.x = 0.0;
        }

        if self.position.y < 0.0 {
            self.position.y = 600.0;
        } else if self.position.y > 600.0 {
            self.position.y = 0.0;
        }
    }
}
