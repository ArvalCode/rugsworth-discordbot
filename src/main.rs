extern crate serenity;

mod events;

use serenity::Client;
use events::Handler;


const TOKEN: &str = "MTA5MjIxNjQ1MDE3NDk1OTY2Nw.GRg9-3.gOV0DrAr4DIYKIySDbP3sYh4vrCL7m2wxQgAdc";

fn main() {
    let mut client = Client::new(TOKEN, Handler)
        .expect("Error creating client");

    if let Err(e) = client.start() {
        println!("Error starting client: {:?}", e);
    }
}
