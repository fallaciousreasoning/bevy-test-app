use bevy::{math::Vec3, prelude::Entity};

pub struct LookAt {
    pub at: Entity,
    pub target_translation: Option<Vec3>
}

impl LookAt {
    pub fn new(at: Entity) -> LookAt {
        LookAt {
            at,
            target_translation: None
        }
    }
}