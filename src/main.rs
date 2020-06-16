#[macro_use] extern crate serenity;
#[macro_use] extern crate serde_json;
extern crate reqwest;
extern crate curl;
extern crate rand;

mod boorus;
mod sadpanda;

use serenity::client::Client;
use serenity::prelude::EventHandler;
use serenity::framework::standard::StandardFramework;
use std::env;

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, ctx: Context, _: Ready) {
        ctx.edit_profile(|e| e.username("Mayumi"));
        
    }
}

fn main() {

    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
        .expect("Error creating client");
    client.with_framework(StandardFramework::new()
                            .configure(|c| c.prefix("~"))
                            .cmd("help", help)
                            .cmd("weiss", weiss)
                            .cmd("weiss2", weiss2)
                            .cmd("midori", midori)
                            .cmd("eve", eve)
                            .cmd("saiyn", saiyn)
                            .cmd("bog", bog)
                            .cmd("bang", bang)
                            .cmd("emote", emote)
                            .cmd("emoji", emote)
                            .cmd("bigsmug", bigsmug)
                            .cmd("chocola2", chocola2)
                            .cmd("vanilla2", vanilla2)
                            .cmd("mikasweat", mikasweat)
                            .cmd("tags", tags)
                            .cmd("safebooru",safebooru)
                            .cmd("danbooru",danbooru)
                            .cmd("gelbooru", gelbooru));
    
    client.
    
    if let Err(why) = client.start() {
        println!("An error occured while running the client: {:?}", why);
    }
}

command!(help(_context, message) {
    let _ = message.channel_id.say("```Current list of commands:\n\t
~emote/~emoji: Display embed image for specified emoji\n\t
~tags: displays embed information for a sadpanda link\n\t

Meme commands:\n\t
~bigsmug: BIG SACH\n\t
~chocola2: BIG CHOCOLA\n\t
~vanilla2: BIG VANILLA\n\t
~mikasweat: :mikasweat:\n\t
~bang: BANG BANG BANG PULL MY DEVIL TRIGGER\n\t

Booru commands (WARNING NSFW):\n\t
~safebooru <tag> [<tag2>]: Displays random image with specified tags from safebooru\n\t
~danbooru <tag> [<tag2>]: Displays random image with specified tags from danbooru\n\t
~gelbooru <tag> [<tag2>]: Displays random image with specified tags from gelbooru\n\t


\n\t```");
                    
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
        Some(t) => boorus::boorus::get_booru_link("danbooru", t, 500),
        None => (String::from("Invalid amount of tags, only 1-2 can be used"), String::from("Invalid")),
    };

    let _ = message.channel_id.say(link);
});

command!(safebooru(_context, message, args) {
    let tag = boorus::boorus::parse_args(args, 100);
    
    let (link, url) = match tag {
        Some(t) => boorus::boorus::get_booru_link("safebooru.org", t, 500),
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
        Some(t) => boorus::boorus::get_booru_link("gelbooru.com", t, 500),
        None => (String::from("Invalid amount of tags, only 1-2 can be used"), String::from("Invalid")),
    };

    let _ = message.channel_id.send_message(|m| m.content(link.as_str())
                .embed(|e| e
                       .image(&url)));

});

command!(weiss(_context, message) {
    let tag = String::from("dark_skin+white_hair+-comic+-guro+-furry+-dark_skinned_male+-male_focus");
    let (link, url) = boorus::boorus::get_booru_link("gelbooru.com", tag, 500);

    let _ = message.channel_id.send_message(|m| m.content(link.as_str())
                .embed(|e| e
                       .image(&url)));
});

command!(weiss2(_context, message) {
    let tag = String::from("dark_skin+soles+feet+-comic+-guro+-dark_skinned_male+-furry+-male_focus");
    let (link, url) = boorus::boorus::get_booru_link("gelbooru.com", tag, 500);

    let _ = message.channel_id.send_message(|m| m.content(link.as_str())
                .embed(|e| e
                       .image(&url)));
});

command!(midori(_context, message) {
    let tag = String::from("miko+armpits+arms_up+-comic+-furry+-male_focus");
    let (link, url) = boorus::boorus::get_booru_link("gelbooru.com", tag, 500);

    let _ = message.channel_id.send_message(|m| m.content(link.as_str())
                .embed(|e| e
                       .image(&url)));
});

command!(eve(_context, message) {
    let tag = String::from("makoto_nanaya");
    let (link, url) = boorus::boorus::get_booru_link("gelbooru.com", tag, 5);

    let _ = message.channel_id.send_message(|m| m.content(link.as_str())
                .embed(|e| e
                       .image(&url)));
});


command!(saiyn(_context, message) {
    let tag = String::from("loli+tan");
    let (link, url) = boorus::boorus::get_booru_link("gelbooru.com", tag, 500);

    let _ = message.channel_id.send_message(|m| m.content(link.as_str())
                .embed(|e| e
                       .image(&url)));
});

   
command!(bog(_context, message) {
    let tag = String::from("loli+smug");
    let (link, url) = boorus::boorus::get_booru_link("gelbooru.com", tag, 500);

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
command!(tags(_context, message, args) {
    let url = args.single::<String>().unwrap();
    let (doujin_title, image_url, doujin_tags) = sadpanda::sadpanda::retrieve_tags(url);    
    let doujin_title = String::from(&doujin_title[1..doujin_title.len()-1]);
    let image_url = String::from(&image_url[1..image_url.len()-1]);
    let _ = message.channel_id.send_message(|m| m.content(doujin_title)
                                                .embed(|e| e.image(&image_url).description(doujin_tags)));
});

command!(bigsmug(_context, message) {
    let url = "https://cdn.discordapp.com/attachments/123165694429888514/520318574343094273/extremelybigsmug4.png";
    let _ = message.channel_id.send_message(|m| m.embed(|e| e.image(&url)));
    
});


command!(chocola2(_context, message) {
    let url = "https://cdn.discordapp.com/attachments/123165694429888514/520318130522685470/extremelybigchocola2.png";
    let _ = message.channel_id.send_message(|m| m.embed(|e| e.image(&url)));
});

command!(vanilla2(_context, message) {
    let url = "https://media.discordapp.net/attachments/123165694429888514/556235851747819522/extremelybigvanilla2.png";
    let _ = message.channel_id.send_message(|m| m.embed(|e| e.image(&url)));
});

command!(mikasweat(_context, message) {
    let url = "https://cdn.discordapp.com/attachments/123165694429888514/542484609967980565/DisOUS0UwAALPh-.png";
    let _ = message.channel_id.send_message(|m| m.embed(|e| e.image(&url)));
});
