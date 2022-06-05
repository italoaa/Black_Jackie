use crate::deck::Card;
use crate::deck::Rank;
use std::io::{self, Write};

pub struct Game {
    pub dealer_hand: Vec<Card>,
    pub player_hand: Vec<Card>,
    pub dealer_score: u8,
    pub player_score: u8,
}

impl Game {
    pub fn new() -> Game {
        let dealerhand = vec![Card::random_card(), Card::random_card()];
        let playerhand = vec![Card::random_card(), Card::random_card()];
        Game {
            dealer_hand: dealerhand.clone(),
            player_hand: playerhand.clone(),
            dealer_score: Game::score(dealerhand),
            player_score: Game::score(playerhand),
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

    pub fn state(&self) {
        println!("dealer:");
        println!("hand:{:?}", self.dealer_hand.iter());
        println!("score:{:?}", self.dealer_score);
        println!("player:");
        println!("hand:{:?}", self.player_hand.iter());
        println!("score:{:?}", self.player_score);
    }

    pub fn prompt(&self) -> String {
        loop {
            println!(
                "your score is {}, Draw or stand(d or s): ",
                self.player_score
            );
            println!("# ");
            io::stdout().flush().expect("cant flush");
            let mut answer: String = String::new();
            io::stdin().read_line(&mut answer).expect("cant read line");
            let answer = answer.trim();
            return answer.to_string();
        }
    }
    //draw for the player
    pub fn draw(&mut self) {
        self.player_hand.push(Card::random_card());
        self.player_score = Game::score(self.player_hand.clone());
    }

    //draw for the dealer
    pub fn drawd(&mut self) {
        self.dealer_hand.push(Card::random_card());
        self.dealer_score = Game::score(self.dealer_hand.clone());
    }
}
