extern crate serenity;

mod events;

use serenity::Client;
use events::Handler;


const TOKEN: &str = ""; //TOKEN

fn main() {
    let mut client = Client::new(TOKEN, Handler)
        .expect("Error creating client");

    if let Err(e) = client.start() {
        println!("Error starting client: {:?}", e);
    }
}
