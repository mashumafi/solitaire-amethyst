use crate::card::{Card, Deck};
use amethyst::ecs::{Component, HashMapStorage};

pub struct DeckComponent {
    deck: Deck,
}

impl DeckComponent {
    pub fn new(deck: Deck) -> DeckComponent {
        DeckComponent { deck }
    }

    pub fn cards(&self) -> &Deck {
        &self.deck
    }

    pub fn cards_mut(&mut self) -> &mut Deck {
        &mut self.deck
    }
}

impl Component for DeckComponent {
    type Storage = HashMapStorage<Self>;
}
