use amethyst::{
    core::math::Vector3,
    ecs::{Component, HashMapStorage},
};

pub struct StackComponent {
    pub original: Option<Vector3<f32>>,
}

impl Default for StackComponent {
    fn default() -> Self {
        StackComponent { original: None }
    }
}

impl Component for StackComponent {
    type Storage = HashMapStorage<Self>;
}
