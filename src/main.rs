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
                            .cmd("help", help)
                            .cmd("weiss", weiss)
                            .cmd("husbando",husbando)
                            .cmd("slut",slut)
                            .cmd("emote", emote)
                            .cmd("emoji", emote)
                            .cmd("safebooru",safebooru)
                            .cmd("danbooru",danbooru));
                            
    if let Err(why) = client.start() {
        println!("An error occured while running the client: {:?}", why);
    }
}

command!(help(_context, message) {
    let _ = message.channel_id.say("```Current list of commands:\n\t
~emote/~emoji: Display URL for specified emoji\n\t
~danbooru <tag>: Displays random image with specified tags from danbooru\n\t
~safebooru <tag>: Displays random image with specified tags from safebooru\n\t
~weiss: posts a nice choco```");
                    
});

command!(husbando(_context, message) {
    let _ = message.channel_id.say("Midori is my husbando!");
});

command!(slut(_context, message) {
    let _ = message.reply("I am not a slut!");
});

command!(danbooru(_context, message, args) {

    let tag = boorus::boorus::parse_args(args);

    let _ = match tag {
        Some(t) => message.channel_id.say((boorus::boorus::get_booru_link("danbooru", t)).as_str()),
        None => message.channel_id.say("Invalid amount of tags, only 1-2 can be used"),
    };
});

command!(safebooru(_context, message, args) {
    let tag = boorus::boorus::parse_args(args);
    
    let _ = match tag {
        Some(t) => message.channel_id.say((boorus::boorus::get_booru_link("safebooru", t)).as_str()),
        None => message.channel_id.say("Invalid amount of tags, only 1-2 can be used"),
    };

});

command!(weiss(_context, message) {
    let tag = String::from("dark_skin+white_hair");
    let _ = message.channel_id.say((boorus::boorus::get_booru_link("danbooru", tag)).as_str());
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
