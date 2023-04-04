use serenity::prelude::*;
use serenity::model::gateway::Ready;
use serenity::model::channel::Message;

pub struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "pong") {
                println!("Error giving message: {:?}", why)
            }
        }
        
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is running", ready.user.name)
    }
}
