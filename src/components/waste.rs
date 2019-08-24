use crate::card::{Card, Deck};
use amethyst::ecs::{Component, HashMapStorage};
use std::mem;

pub struct WasteComponent {
    deck: Deck,
}

impl Default for WasteComponent {
    fn default() -> Self {
        WasteComponent {
            deck: Deck::default(),
        }
    }
}

impl WasteComponent {
    pub fn insert(&mut self, card: Card) {
        self.deck.insert(card);
    }

    pub fn swap(&mut self, deck: &mut Deck) {
        mem::swap(&mut self.deck, deck);
    }
}

impl Component for WasteComponent {
    type Storage = HashMapStorage<Self>;
}
