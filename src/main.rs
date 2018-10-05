#[macro_use] extern crate serenity;
extern crate curl;
extern crate serde_json;

mod boorus;

use serenity::client::Client;
use serenity::prelude::EventHandler;
use serenity::framework::standard::StandardFramework;
use std::env;

struct Handler;

impl EventHandler for Handler {}

fn main() {

    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
        .expect("Error creating client");
    client.with_framework(StandardFramework::new()
                            .configure(|c| c.prefix("~"))
                            .cmd("ping",ping)
                            .cmd("slut",slut)
                            .cmd("danbooru",danbooru));
    if let Err(why) = client.start() {
        println!("An error occured while running the client: {:?}", why);
    }
}

command!(ping(_context, message) {
    let _ = message.reply("Pong!");
});

command!(slut(_context, message) {

    let _ = message.reply("I am not a slut!");
});

command!(danbooru(_context, message, args) {
    let tag = args.single::<String>().unwrap();
    let _ = message.reply((boorus::boorus::get_booru_link("danbooru", tag)).as_str()); 
});
