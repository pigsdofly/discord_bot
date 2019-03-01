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
                            .cmd("weiss2", weiss2)
                            .cmd("midori", midori)
                            .cmd("bang", bang)
                            .cmd("husbando",husbando)
                            .cmd("slut",slut)
                            .cmd("emote", emote)
                            .cmd("emoji", emote)
                            .cmd("bigsmug", bigsmug)
                            .cmd("chocola2", chocola2)
                            .cmd("safebooru",safebooru)
                            .cmd("danbooru",danbooru)
                            .cmd("gelbooru", gelbooru));
                            
    if let Err(why) = client.start() {
        println!("An error occured while running the client: {:?}", why);
    }
}

command!(help(_context, message) {
    let _ = message.channel_id.say("```Current list of commands:\n\t
~emote/~emoji: Display URL for specified emoji\n\t
~bigsmug: BIG SACH\n\t
~chocola2: BIG CHOCOLA\n\t
~danbooru <tag> [<tag2>]: Displays random image with specified tags from danbooru\n\t
~safebooru <tag> [<tag2>]: Displays random image with specified tags from safebooru\n\t
~gelbooru <tag> [<tag2>]: Displays random image with specified tags from gelbooru\n\t
~bang: BANG BANG BANG PULL MY DEVIL TRIGGER\n\t
~weiss: posts a nice choco
~weiss2: footfags smh```");
                    
});

command!(husbando(_context, message) {
    let _ = message.channel_id.say("Midori is my husbando!");
});

command!(slut(_context, message) {
    let _ = message.reply("I am not a slut!");
});

command!(danbooru(_context, message, args) {

    let tag = boorus::boorus::parse_args(args, 2);

    let (link, _url) = match tag {
        Some(t) => boorus::boorus::get_booru_link("danbooru", t),
        None => (String::from("Invalid amount of tags, only 1-2 can be used"), String::from("Invalid")),
    };

    let _ = message.channel_id.say(link);
});

command!(safebooru(_context, message, args) {
    let tag = boorus::boorus::parse_args(args, 100);
    
    let (link, url) = match tag {
        Some(t) => boorus::boorus::get_booru_link("safebooru.org", t),
        None => (String::from("Invalid amount of tags, only 1-2 can be used"), String::from("Invalid")),
    };

    let url = format!("https:{}",url);
    let _ = message.channel_id.send_message(|m| m.content(link.as_str())
                .embed(|e| e
                       .image(url)));

});

command!(gelbooru(_context, message, args) {
    let tag = boorus::boorus::parse_args(args, 100);
    
    let (link, url) = match tag {
        Some(t) => boorus::boorus::get_booru_link("gelbooru.com", t),
        None => (String::from("Invalid amount of tags, only 1-2 can be used"), String::from("Invalid")),
    };

    let _ = message.channel_id.send_message(|m| m.content(link.as_str())
                .embed(|e| e
                       .image(&url)));

});

command!(weiss(_context, message) {
    let tag = String::from("dark_skin+white_hair+-comic+-guro+-furry+-dark_skinned_male+-male_focus");
    let (link, url) = boorus::boorus::get_booru_link("gelbooru.com", tag);

    let _ = message.channel_id.send_message(|m| m.content(link.as_str())
                .embed(|e| e
                       .image(&url)));
});

command!(weiss2(_context, message) {
    let tag = String::from("dark_skin+soles+feet+-comic+-guro+-dark_skinned_male+-furry+-male_focus");
    let (link, url) = boorus::boorus::get_booru_link("gelbooru.com", tag);

    let _ = message.channel_id.send_message(|m| m.content(link.as_str())
                .embed(|e| e
                       .image(&url)));
});

command!(midori(_context, message) {
    let tag = String::from("miko+armpits+arms_up+-comic+-furry+-male_focus");
    let (link, url) = boorus::boorus::get_booru_link("gelbooru.com", tag);

    let _ = message.channel_id.send_message(|m| m.content(link.as_str())
                .embed(|e| e
                       .image(&url)));
});


command!(bang(_context, message) {
    let _ = message.channel_id.send_message(|m| m.content("BANG BANG BANG https://www.youtube.com/watch?v=iUVDHEGR31k"));
    
});

command!(emote(_context, message, args) {
    let emoji_name = args.single::<String>().unwrap();
    let emoji_name = String::from(&emoji_name[2..emoji_name.len()-1]);
    let split_emoji : Vec<&str> = emoji_name.split(":").collect();

    let (emoji_id, filetype) = match split_emoji.len() {
        2 => (split_emoji[1], "png"),
        3 => (split_emoji[2], "gif"),
        _ => ("Bad input", ""),
    };

    if split_emoji.len() < 2 {
        let _ = message.channel_id.say("Bad input");
    } else {

        let url = format!("https://cdn.discordapp.com/emojis/{}.{}", emoji_id, filetype);
        let _ = message.channel_id.send_message(|m| m.embed(|e| e.image(&url)));
    }

});

command!(bigsmug(_context, message) {
    let url = "https://cdn.discordapp.com/attachments/123165694429888514/520318574343094273/extremelybigsmug4.png";
    let _ = message.channel_id.send_message(|m| m.embed(|e| e.image(&url)));
    
});

command!(chocola2(_context, message) {
    let url = "https://cdn.discordapp.com/attachments/123165694429888514/520318130522685470/extremelybigchocola2.png";
    let _ = message.channel_id.send_message(|m| m.embed(|e| e.image(&url)));
});

