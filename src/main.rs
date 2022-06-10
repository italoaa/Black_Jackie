use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

use std::net::TcpStream;
use std::sync::Arc;

mod game;

use game::Game;

struct State {
    players: u8,
    ready: Vec<bool>,
    scores: Vec<u8>,
}

impl State {
    fn new() -> State {
        State {
            players: 2,
            ready: vec![],
            scores: vec![],
        }
    }
    fn set_ready(&mut self) {
        self.ready.push(true);
    }
    fn all_ready(&self) -> bool {
        let mut count = 0;
        for r in self.ready.iter() {
            if *r == true {
                count += 1;
            }
        }
        if count == self.players {
            true
        } else {
            false
        }
    }
    fn reset_ready(&mut self) {
        self.ready = vec![];
    }

    fn add_score(&mut self, score: u8) -> usize {
        self.scores.push(score);
        return self.scores.len() - 1;
    }
    fn heighest_score(&self, score: u8, id: usize) -> bool {
        if id == 1000 {
            println!("{:?}", self.scores);
            return false;
        }
        for value in self.scores.iter() {
            if *value == self.scores[id] {
                // the value is the current player score
                continue;
            } else if *value > score {
                // There is another player with a higher score
                return false;
            } else if *value < score {
                // This value is lower so we can continue to the next
                continue;
            }
        }
        return true;
    }
}

struct Connection {
    stream: TcpStream,
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:6000").await.unwrap();
    let state = Arc::new(Mutex::new(State::new()));

    loop {
        let (mut stream, _addr) = listener.accept().await.unwrap();
        let state = Arc::clone(&state);
        tokio::spawn(async move {
            let (reader, mut writer) = stream.split();
            let mut line: String;
            let mut waiting: bool;

            let mut reader = BufReader::new(reader);

            // ask for ready (there has to be a better way)
            loop {
                line = "Are you ready? [y/n] \n> ".to_string();
                writer.write(b"Are you ready? [y/n] \n> ").await.unwrap();
                line.clear();

                reader.read_line(&mut line).await.unwrap();

                if line.trim_end() == "y" {
                    writer.write(b"You are ready!\n").await.unwrap();
                    let mut state = state.lock().await;
                    state.set_ready();
                    println!("{} : {:?}", state.all_ready(), state.ready);
                    if !state.all_ready() {
                        writer.write(b"Wait for other player").await.unwrap();
                        waiting = true;
                        break;
                    } else {
                        writer.write(b"All ready!\n").await.unwrap();
                        waiting = false;
                        break;
                    }
                } else if line.trim_end() == "n" {
                    writer.write(b"not ready\n").await.unwrap();
                    continue;
                } else {
                    writer.write(b"wat? try y or n\n").await.unwrap();
                    continue;
                }
            }

            // if client ready wait for the rest to be ready
            if waiting {
                loop {
                    sleep(Duration::from_millis(500)).await;
                    let mut state = state.lock().await;
                    if state.all_ready() {
                        // all ready
                        writer.write(b"\nAll ready!\n").await.unwrap();
                        state.reset_ready();
                        drop(state);
                        break;
                    } else {
                        writer.write(b".").await.unwrap();
                        drop(state);
                    }
                }
            }

            // Client ready
            // Game Starts
            // Display initial state
            // Display one card to the other player
            // init the game
            let mut game = Game::new();
            let mut id: usize;
            waiting = false;
            loop {
                // display the cards of the player
                //
                // ask to stand or hit
                let cards: String = format!(
                    "[*] You have a hand of: {:?}\n[*] That hand has a value of {}\n",
                    game.player_hand, game.player_score
                );
                writer.write(cards.as_bytes()).await.unwrap();
                writer
                    .write(b"\n[*] Do you wish to Hit(h) or Stand(s)\n> ")
                    .await
                    .unwrap();
                line.clear();
                // get response
                reader.read_line(&mut line).await.unwrap();
                if line.trim_end() == "h" {
                    // hit
                    game.drawp();

                    // Checks
                    if game.player_score > 21 {
                        // Bust
                        writer.write(b"Busted\n").await.unwrap();
                        id = 1000;
                        game.busted = true;
                        let mut state = state.lock().await;
                        state.set_ready();
                        if !state.all_ready() {
                            drop(state);
                            writer.write(b"Wait for other player").await.unwrap();
                            waiting = true;
                            break;
                        }
                        drop(state);
                        break;
                    } else {
                        // no bust may hit again
                        writer.write(b"Did not bust!\n").await.unwrap();
                        let mut state = state.lock().await;
                        id = state.add_score(game.player_score);
                        drop(state);
                        continue;
                    }
                } else {
                    // stand
                    writer.write(b"Stand\n").await.unwrap();
                    let mut state = state.lock().await;
                    id = state.add_score(game.player_score);
                    state.set_ready();
                    if !state.all_ready() {
                        drop(state);
                        writer.write(b"Wait for other player").await.unwrap();
                        waiting = true;
                        break;
                    }
                    drop(state);
                    break;
                }
            }
            // Both clients play until they stand

            if waiting {
                loop {
                    sleep(Duration::from_millis(500)).await;
                    let mut state = state.lock().await;
                    if state.all_ready() {
                        // all ready
                        writer.write(b"\nAll ready!\n").await.unwrap();
                        state.reset_ready();
                        drop(state);
                        break;
                    } else {
                        writer.write(b".").await.unwrap();
                        drop(state);
                    }
                }
            }
            // Wait for the other player

            if game.busted {
                // Busted and the other player wins
                writer.write(b"You lost!\n").await.unwrap();
            } else {
                // not busted
                let state = state.lock().await;
                if state.heighest_score(game.player_score, id) {
                    writer.write(b"You won!\n").await.unwrap();
                } else {
                    writer.write(b"You lost!\n").await.unwrap();
                }
            }
            // Then both hands are compared
        });
    }
}
