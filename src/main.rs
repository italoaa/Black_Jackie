use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

use std::sync::Arc;

struct State {
    players: u8,
    ready: Vec<bool>,
}

impl State {
    fn new() -> State {
        State {
            players: 2,
            ready: vec![],
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
            let waiting: bool;

            let mut reader = BufReader::new(reader);

            // ask for ready (there has to be a better way)
            loop {
                line = "Are you ready? [y/n] \n> ".to_string();
                writer.write(line.as_bytes()).await.unwrap();
                line.clear();

                reader.read_line(&mut line).await.unwrap();

                if line.trim_end() == "y" {
                    line = "You are ready!\n".to_string();
                    writer.write(line.as_bytes()).await.unwrap();
                    line.clear();
                    let mut state = state.lock().await;
                    state.set_ready();
                    println!("{} : {:?}", state.all_ready(), state.ready);
                    if !state.all_ready() {
                        line = "Wait for other player".to_string();
                        writer.write(line.as_bytes()).await.unwrap();
                        line.clear();
                        waiting = true;
                        break;
                    } else {
                        line = "All ready!".to_string();
                        writer.write(line.as_bytes()).await.unwrap();
                        line.clear();
                        waiting = false;
                        break;
                    }
                } else if line.trim_end() == "n" {
                    line = "not ready\n".to_string();
                    writer.write(line.as_bytes()).await.unwrap();
                    line.clear();
                    continue;
                } else {
                    line = "wat? try y or n\n".to_string();
                    writer.write(line.as_bytes()).await.unwrap();
                    line.clear();
                    continue;
                }
            }

            // if client ready wait for the rest to be ready
            if waiting {
                loop {
                    sleep(Duration::from_millis(500)).await;
                    let state = state.lock().await;
                    if state.all_ready() {
                        // all ready
                        line = "\nAll ready!\n".to_string();
                        writer.write(line.as_bytes()).await.unwrap();
                        break;
                    } else {
                        line = ".".to_string();
                        writer.write(line.as_bytes()).await.unwrap();
                        drop(state);
                    }
                }
            }
            // Client ready
            // Game Starts
            // Display one card to the other player
            // Both clients play until they stand
            // Then both hands are compared
        });
    }
}
