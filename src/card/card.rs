#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Color {
    Black,
    Red,
}

impl Color {}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Suit {
    Spade,
    Heart,
    Club,
    Diamond,
}

#[derive(Copy, Clone)]
pub struct SuitIterator {
    suit: Option<Suit>,
}

impl Default for SuitIterator {
    fn default() -> Self {
        SuitIterator { suit: None }
    }
}

impl Iterator for SuitIterator {
    type Item = Suit;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(suit) = match &self.suit {
            Some(suit) => match suit {
                Suit::Spade => Some(Suit::Heart),
                Suit::Heart => Some(Suit::Club),
                Suit::Club => Some(Suit::Diamond),
                Suit::Diamond => None,
            },
            None => Some(Suit::Spade),
        } {
            self.suit = Some(suit);
            self.suit
        } else {
            None
        }
    }
}

impl Suit {
    pub fn color(&self) -> Color {
        match self {
            Suit::Spade => Color::Black,
            Suit::Heart => Color::Red,
            Suit::Club => Color::Black,
            Suit::Diamond => Color::Red,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Rank {
    pub fn next(&self) -> Option<Rank> {
        match self {
            Rank::Ace => Some(Rank::Two),
            Rank::Two => Some(Rank::Three),
            Rank::Three => Some(Rank::Four),
            Rank::Four => Some(Rank::Five),
            Rank::Five => Some(Rank::Six),
            Rank::Six => Some(Rank::Seven),
            Rank::Seven => Some(Rank::Eight),
            Rank::Eight => Some(Rank::Nine),
            Rank::Nine => Some(Rank::Ten),
            Rank::Ten => Some(Rank::Jack),
            Rank::Jack => Some(Rank::Queen),
            Rank::Queen => Some(Rank::King),
            Rank::King => None,
        }
    }

    pub fn is_next(&self, other: &Rank) -> bool {
        match other.next() {
            Some(rank) => *self == rank,
            None => false,
        }
    }

    pub fn is_king(&self) -> bool {
        match self {
            Rank::King => true,
            _ => false,
        }
    }

    pub fn is_ace(&self) -> bool {
        match self {
            Rank::Ace => true,
            _ => false,
        }
    }
}

#[derive(Copy, Clone)]
pub struct RankIterator {
    rank: Option<Rank>,
}

impl Default for RankIterator {
    fn default() -> Self {
        RankIterator { rank: None }
    }
}

impl Iterator for RankIterator {
    type Item = Rank;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.rank {
            Some(rank) => match rank.next() {
                Some(rank) => {
                    self.rank = Some(rank);
                    self.rank
                }
                None => None,
            },
            None => {
                self.rank = Some(Rank::Ace);
                self.rank
            }
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Card {
        Card { rank, suit }
    }

    pub fn rank(&self) -> Rank {
        self.rank
    }

    pub fn suit(&self) -> Suit {
        self.suit
    }

    pub fn index(&self) -> usize {
        (self.rank as usize) + (self.suit as usize) * 13
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_cmp() {
        assert_eq!(Color::Red, Color::Red);
        assert_eq!(Color::Black, Color::Black);
        assert_ne!(Color::Red, Color::Black);
        assert_ne!(Color::Black, Color::Red);
    }

    #[test]
    fn test_rank_iterator() {
        let mut iter = RankIterator::default();
        assert_eq!(Some(Rank::Ace), iter.next());
        assert_eq!(Some(Rank::Two), iter.next());
        assert_eq!(Some(Rank::Three), iter.next());
        assert_eq!(Some(Rank::Four), iter.next());
        assert_eq!(Some(Rank::Five), iter.next());
        assert_eq!(Some(Rank::Six), iter.next());
        assert_eq!(Some(Rank::Seven), iter.next());
        assert_eq!(Some(Rank::Eight), iter.next());
        assert_eq!(Some(Rank::Nine), iter.next());
        assert_eq!(Some(Rank::Ten), iter.next());
        assert_eq!(Some(Rank::Jack), iter.next());
        assert_eq!(Some(Rank::Queen), iter.next());
        assert_eq!(Some(Rank::King), iter.next());
        assert_eq!(None, iter.next());
    }
    #[test]
    fn test_suit_iterator() {
        let mut iter = SuitIterator::default();
        assert_eq!(Some(Suit::Spade), iter.next());
        assert_eq!(Some(Suit::Heart), iter.next());
        assert_eq!(Some(Suit::Club), iter.next());
        assert_eq!(Some(Suit::Diamond), iter.next());
        assert_eq!(None, iter.next());
    }
}
