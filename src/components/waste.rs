use crate::card::{Card, Deck};
use amethyst::ecs::{Component, HashMapStorage};

pub struct WasteComponent {
    cards: Vec<Card>,
}

impl Default for WasteComponent {
    fn default() -> Self {
        WasteComponent {
            cards: Vec::with_capacity(24),
        }
    }
}

impl WasteComponent {
    pub fn insert(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn swap(&mut self, deck: &mut Deck) {
        deck.swap(&mut self.cards);
    }
}

impl Component for WasteComponent {
    type Storage = HashMapStorage<Self>;
}
