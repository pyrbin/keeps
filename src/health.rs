use crate::prelude::*;

#[derive(Component)]
pub struct Health {
    pub max: f32,
    pub current: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { max, current: max }
    }

    pub fn damage(&mut self, damage: f32) {
        if self.current > 0. {
            self.current -= damage;
        }
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0.
    }
}
