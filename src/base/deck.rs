use rand::Rng;

#[derive(Debug, Clone)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

#[derive(Debug, Clone)]
pub enum Rank {
    ACE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    TEN,
    JACK,
    QUEEN,
    KING,
}
#[derive(Debug, Clone)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Card {
        Card {
            rank: rank,
            suit: suit,
        }
    }

    pub fn random_card() -> Card {
        let rank = match rand::thread_rng().gen_range(1..=13) {
            1 => Rank::ACE,
            2 => Rank::TWO,
            3 => Rank::THREE,
            4 => Rank::FOUR,
            5 => Rank::FIVE,
            6 => Rank::SIX,
            7 => Rank::SEVEN,
            8 => Rank::EIGHT,
            9 => Rank::NINE,
            10 => Rank::TEN,
            11 => Rank::JACK,
            12 => Rank::QUEEN,
            13 => Rank::KING,
            _ => panic!("not in range"),
        };
        let suit = match rand::thread_rng().gen_range(1..=4) {
            1 => Suit::Clubs,
            2 => Suit::Diamonds,
            3 => Suit::Hearts,
            4 => Suit::Spades,
            _ => panic!("not in range"),
        };
        Card::new(rank, suit)
    }
}
