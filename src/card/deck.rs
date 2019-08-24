use crate::card::{Card, RankIterator, SuitIterator};

pub struct Deck {
    cards: Vec<Card>,
}

impl Default for Deck {
    fn default() -> Self {
        let mut cards = Vec::with_capacity(52);
        for suit in SuitIterator::default() {
            for rank in RankIterator::default() {
                cards.push(Card::new(rank, suit));
            }
        }
        Deck { cards }
    }
}

impl Deck {
    pub fn shuffle(&mut self) {}

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn insert(&mut self, card: Card) {
        self.cards.push(card);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size() {
        let deck = Deck::default();
        assert_eq!(52, deck.cards.len());
    }

    #[test]
    fn test_order() {
        let deck = Deck::default();
        for (i, card) in deck.cards.iter().enumerate() {
            assert_eq!(i, card.index());
        }
    }

    #[test]
    fn test_shuffle() {}
}
