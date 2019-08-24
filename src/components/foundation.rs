use amethyst::ecs::{Component, DenseVecStorage};
use crate::card::Deck;

pub struct FoundationComponent {
    deck: Deck
}

impl Default for FoundationComponent {
    fn default() -> Self {
        FoundationComponent { deck: Deck::default() }
    }
}

impl Component for FoundationComponent {
    type Storage = DenseVecStorage<Self>;
}
