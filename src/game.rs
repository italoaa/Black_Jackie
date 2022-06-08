pub struct Game {
    pub player_hand: Vec<Card>,
    pub player_score: u8,
    pub busted: bool,
}

impl Game {
    pub fn new() -> Game {
        let playerhand = vec![Card::random_card(), Card::random_card()];
        Game {
            player_hand: playerhand.clone(),
            player_score: Game::score(playerhand),
            busted: false,
        }
    }

    pub fn score(hand: Vec<Card>) -> u8 {
        let mut score = 0;
        for card in hand.iter() {
            score += match card.rank {
                Rank::ACE => 1,
                Rank::TWO => 2,
                Rank::THREE => 3,
                Rank::FOUR => 4,
                Rank::FIVE => 5,
                Rank::SIX => 6,
                Rank::SEVEN => 7,
                Rank::EIGHT => 8,
                Rank::NINE => 9,
                Rank::TEN => 10,
                Rank::JACK => 10,
                Rank::QUEEN => 10,
                Rank::KING => 10,
            };
        }
        return score;
    }

    //draw for the player
    pub fn drawp(&mut self) {
        self.player_hand.push(Card::random_card());
        self.player_score = Game::score(self.player_hand.clone());
    }
}

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
