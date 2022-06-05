mod game;
use game::Game;

mod deck;

fn main() {
    // Start game
    let mut game: Game = Game::new();
    // Display starting game state
    game.state();
    // loop
    loop {
        // Prompt for user input
        let answer = Game::prompt(&game);
        // If draw
        if answer == "d" {
            // take a card
            Game::draw(&mut game);
            if game.player_score > 21 {
                // check if bust
                // lost
                println!("lost")
                // end game
            } else {
                println!("no bust continue");
                continue;
            }
            // ask to draw or stand
        } else if answer == "s" {
            // if stand
            // check values of dealer vs the player
            if game.dealer_score > game.player_score {
                // if player < dealer lost
                println!("dealer has more score you lost");
            } else if game.player_score == game.dealer_score {
                // if = then tie
                println!("tie");
            }
        } else if game.player_score > game.dealer_score {
            // this means that the dealer has less than the player so
            while game.dealer_score < game.player_score {
                // draw
                Game::drawd(&mut game);
                // check bust
                if game.dealer_score > 21 {
                    println!("u won")
                } else if game.dealer_score == game.player_score {
                    // check tie
                    println!("tie")
                } else if game.dealer_score > game.player_score {
                    // check dealer got closer to 21
                    println!("lost")
                }
            }
        }
    }
}
