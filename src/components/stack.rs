use amethyst::{
    core::math::Vector3,
    ecs::{Component, HashMapStorage},
};

pub struct StackComponent {
    pub original: Vector3<f32>,
}

impl StackComponent {
    pub fn new(original: Vector3<f32>) -> Self {
        StackComponent { original }
    }
}

impl Component for StackComponent {
    type Storage = HashMapStorage<Self>;
}
