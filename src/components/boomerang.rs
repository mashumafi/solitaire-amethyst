use amethyst::{
    core::math::Vector3,
    ecs::{Component, DenseVecStorage},
};

pub struct BoomerangComponent {
    pub original: Option<Vector3<f32>>,
}

impl Default for BoomerangComponent {
    fn default() -> Self {
        BoomerangComponent { original: None }
    }
}

impl Component for BoomerangComponent {
    type Storage = DenseVecStorage<Self>;
}
