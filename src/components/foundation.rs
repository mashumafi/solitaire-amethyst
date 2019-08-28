use crate::card::Card;
use amethyst::ecs::{Component, DenseVecStorage};

pub struct FoundationComponent {
    cards: Vec<Card>,
}

impl Default for FoundationComponent {
    fn default() -> Self {
        FoundationComponent {
            cards: Vec::with_capacity(13),
        }
    }
}

impl Component for FoundationComponent {
    type Storage = DenseVecStorage<Self>;
}
