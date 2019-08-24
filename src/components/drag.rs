use amethyst::ecs::{Component, DenseVecStorage};

pub struct DragComponent {
    pub selected: bool,
}

impl Default for DragComponent {
    fn default() -> Self {
        DragComponent { selected: false }
    }
}

impl Component for DragComponent {
    type Storage = DenseVecStorage<Self>;
}
