#[macro_use] extern crate serenity;
extern crate curl;
extern crate rand;

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
                            .cmd("emote", emote)
                            .cmd("emoji", emote)
                            .cmd("safebooru",safebooru)
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
    let _ = message.channel_id.say((boorus::boorus::get_booru_link("danbooru", tag)).as_str()); 
});

command!(safebooru(_context, message, args) {
    let tag = args.single::<String>().unwrap();
    let _ = message.channel_id.say((boorus::boorus::get_booru_link("safebooru", tag)).as_str()); 
});

command!(emote(_context, message, args) {
    let emoji_name = args.single::<String>().unwrap();
    let emoji_name = String::from(&emoji_name[2..emoji_name.len()-1]);
    let split_emoji : Vec<&str> = emoji_name.split(":").collect();

    if split_emoji.len() < 2 {
        let _ = message.channel_id.say("Bad input");
    } else {

        let emoji_id = split_emoji[1];

        let url = format!("https://cdn.discordapp.com/emojis/{}.png", emoji_id);
        let _ = message.channel_id.say(url);
    }

});
