pub struct Asteroids {
    should_continue: bool,
}

impl Asteroids {
    pub fn new() -> Asteroids {
        Asteroids {
            should_continue: true,
        }
    }

    pub fn should_continue(&self) -> bool {
        self.should_continue
    }
}

pub fn update_and_render(asteroids: &mut Asteroids, input: &[char]) {
    if input.iter().any(|&i| i == 'q') {
        asteroids.should_continue = false;
        return
    }
    println!("Update");
    println!("Render");
}
