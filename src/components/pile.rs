use crate::card::{Card, Deck};
use amethyst::ecs::{Component, DenseVecStorage};

pub enum PileComponent {
    Waste(Waste),
    Tableau,
    Foundation(Foundation),
}

impl PileComponent {
    pub fn waste() -> PileComponent {
        PileComponent::Waste(Waste::default())
    }

    pub fn foundation() -> PileComponent {
        PileComponent::Foundation(Foundation::default())
    }

    pub fn tableau() -> PileComponent {
        PileComponent::Tableau
    }
}

impl Component for PileComponent {
    type Storage = DenseVecStorage<Self>;
}

pub struct Waste {
    cards: Vec<Card>,
}

impl Default for Waste {
    fn default() -> Self {
        Waste {
            cards: Vec::with_capacity(24),
        }
    }
}

impl Waste {
    pub fn insert(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn swap(&mut self, deck: &mut Deck) {
        deck.swap(&mut self.cards);
    }
}

pub struct Foundation {
    cards: Vec<Card>,
}

impl Default for Foundation {
    fn default() -> Self {
        Foundation {
            cards: Vec::with_capacity(13),
        }
    }
}
