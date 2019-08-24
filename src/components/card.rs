use crate::card::Card;
use amethyst::ecs::{Component, DenseVecStorage};

pub enum CardState {
    Waste,
    Tableau,
    Foundation,
    Drag,
}

pub struct CardComponent {
    pub card: Card,
    pub state: CardState,
}

impl CardComponent {
    pub fn new(card: Card, state: CardState) -> Self {
        CardComponent { card, state }
    }
}

impl Component for CardComponent {
    type Storage = DenseVecStorage<Self>;
}
