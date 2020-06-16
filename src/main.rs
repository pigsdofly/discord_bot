#[macro_use] extern crate serde_json;
extern crate serenity;
extern crate reqwest;
extern crate curl;
extern crate rand;

mod boorus;
mod sadpanda;
mod commands;

use serenity::client::{Client, Context};
use serenity::model::gateway::{Ready, Activity};
use serenity::prelude::EventHandler;
use serenity::framework::standard::{
    StandardFramework,
    macros::{group, help},
};

use std::env;

use commands::commands::*;

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, ctx: Context, _: Ready) {
        ctx.set_activity(Activity::playing("https://www.youtube.com/watch?v=r3j1IhPRFs0"));
        
    }
}

#[group]
#[commands(help, danbooru, gelbooru, safebooru, weiss, weiss2, midori, bog, eve, saiyn, bang, emote, tags, bigsmug, chocola2, vanilla2, mikasweat)]
struct Cmd;

fn main() {
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
        .expect("Error creating client");
    client.with_framework(StandardFramework::new()
                            .configure(|c| c.prefix("~"))
                            .group(&CMD_GROUP));
    
    if let Err(why) = client.start() {
        println!("An error occured while running the client: {:?}", why);
    }
}
