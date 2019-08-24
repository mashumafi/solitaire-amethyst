use crate::card::Deck;
use amethyst::ecs::{Component, NullStorage};

pub struct TableauComponent {}

impl Default for TableauComponent {
    fn default() -> Self {
        TableauComponent {}
    }
}

impl Component for TableauComponent {
    type Storage = NullStorage<Self>;
}
