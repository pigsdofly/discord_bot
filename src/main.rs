#[macro_use] extern crate serenity;
#[macro_use] extern crate serde_json;
extern crate reqwest;
extern crate curl;
extern crate rand;

mod boorus;
mod sadpanda;

use serenity::client::{Client, Context};
use serenity::model::{channel::{Channel, Message}, gateway::{Ready, Activity}};
use serenity::prelude::EventHandler;
use serenity::framework::standard::{
    StandardFramework,
    Args, CheckResult, CommandOptions, CommandResult,
    macros::{command, group, help},
};

use std::env;

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
#[command]
fn help(ctx: &mut Context, message: &Message) -> CommandResult {
    let _ = message.channel_id.say(&ctx.http,"```Current list of commands:\n\t
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

    Ok(())
                    
}

#[command]
fn danbooru(ctx: &mut Context, message: &Message, args: Args) -> CommandResult {

    let tag = boorus::boorus::parse_args(args, 2);

    let (link, _url) = match tag {
        Some(t) => boorus::boorus::get_booru_link("danbooru", t, 500),
        None => (String::from("Invalid amount of tags, only 1-2 can be used"), String::from("Invalid")),
    };

    let _ = message.channel_id.say(&ctx.http,link);
    
    Ok(())
}

#[command]
fn safebooru(ctx: &mut Context, message: &Message, args: Args) -> CommandResult {
    let tag = boorus::boorus::parse_args(args, 100);
    
    let (link, url) = match tag {
        Some(t) => boorus::boorus::get_booru_link("safebooru.org", t, 500),
        None => (String::from("Invalid amount of tags, only 1-2 can be used"), String::from("Invalid")),
    };

    let url = format!("https:{}",url);
    let _ = message.channel_id.send_message(&ctx.http, |m| m.content(link.as_str())
                .embed(|e| e
                       .image(url)));
    Ok(())
}

#[command]

fn gelbooru(ctx: &mut Context, message: &Message, args: Args) -> CommandResult {
    let tag = boorus::boorus::parse_args(args, 100);
    
    let (link, url) = match tag {
        Some(t) => boorus::boorus::get_booru_link("gelbooru.com", t, 500),
        None => (String::from("Invalid amount of tags, only 1-2 can be used"), String::from("Invalid")),
    };

    let _ = message.channel_id.send_message(&ctx.http, |m| m.content(link.as_str())
                .embed(|e| e
                       .image(&url)));
    Ok(())
}


#[command]
fn weiss(ctx: &mut Context, message: &Message) -> CommandResult {
    let tag = String::from("dark_skin+white_hair+-comic+-guro+-furry+-dark_skinned_male+-male_focus");
    let (link, url) = boorus::boorus::get_booru_link("gelbooru.com", tag, 500);

    let _ = message.channel_id.send_message(&ctx.http, |m| m.content(link.as_str())
                .embed(|e| e
                       .image(&url)));
    Ok(())
}

#[command]
fn weiss2(ctx: &mut Context, message: &Message) -> CommandResult {
    let tag = String::from("dark_skin+soles+feet+-comic+-guro+-dark_skinned_male+-furry+-male_focus");
    let (link, url) = boorus::boorus::get_booru_link("gelbooru.com", tag, 500);

    let _ = message.channel_id.send_message(&ctx.http, |m| m.content(link.as_str())
                .embed(|e| e
                       .image(&url)));
                       
    Ok(())
}

#[command]
fn midori(ctx: &mut Context, message: &Message) -> CommandResult {
    let tag = String::from("miko+armpits+arms_up+-comic+-furry+-male_focus");
    let (link, url) = boorus::boorus::get_booru_link("gelbooru.com", tag, 500);

    let _ = message.channel_id.send_message(&ctx.http, |m| m.content(link.as_str())
                .embed(|e| e
                       .image(&url)));
                       
    Ok(())
}

#[command]
fn eve(ctx: &mut Context, message: &Message) -> CommandResult {
    let tag = String::from("makoto_nanaya");
    let (link, url) = boorus::boorus::get_booru_link("gelbooru.com", tag, 5);

    let _ = message.channel_id.send_message(&ctx.http, |m| m.content(link.as_str())
                .embed(|e| e
                       .image(&url)));
                       
    Ok(())
}



#[command]
fn saiyn(ctx: &mut Context, message: &Message) -> CommandResult {
    let tag = String::from("loli+tan");
    let (link, url) = boorus::boorus::get_booru_link("gelbooru.com", tag, 500);

    let _ = message.channel_id.send_message(&ctx.http, |m| m.content(link.as_str())
                .embed(|e| e
                       .image(&url)));
                       
    Ok(())
}

   

#[command]
fn bog(ctx: &mut Context, message: &Message) -> CommandResult {
    let tag = String::from("loli+smug");
    let (link, url) = boorus::boorus::get_booru_link("gelbooru.com", tag, 500);

    let _ = message.channel_id.send_message(&ctx.http, |m| m.content(link.as_str())
                .embed(|e| e
                       .image(&url)));

    Ok(())
}


#[command]
fn bang(ctx: &mut Context, message: &Message) -> CommandResult {
    let _ = message.channel_id.send_message(&ctx.http, |m| m.content("BANG BANG BANG https://www.youtube.com/watch?v=iUVDHEGR31k"));
    
    Ok(())
}


#[command]
#[aliases("emoji")]
fn emote(ctx: &mut Context, message: &Message, mut args: Args) -> CommandResult {
    let emoji_name = args.single::<String>().unwrap();
    let emoji_name = String::from(&emoji_name[2..emoji_name.len()-1]);
    let split_emoji : Vec<&str> = emoji_name.split(":").collect();

    let (emoji_id, filetype) = match split_emoji.len() {
        2 => (split_emoji[1], "png"),
        3 => (split_emoji[2], "gif"),
        _ => ("Bad input", ""),
    };

    if split_emoji.len() < 2 {
        let _ = message.channel_id.say(&ctx.http,"Bad input");
    } else {

        let url = format!("https://cdn.discordapp.com/emojis/{}.{}", emoji_id, filetype);
        let _ = message.channel_id.send_message(&ctx.http, |m| m.embed(|e| e.image(&url)));
    }

    Ok(())
}

#[command]
fn tags(ctx: &mut Context, message: &Message, mut args: Args) -> CommandResult {
    let url = args.single::<String>().unwrap();
    let (doujin_title, image_url, doujin_tags) = sadpanda::sadpanda::retrieve_tags(url);    
    let doujin_title = String::from(&doujin_title[1..doujin_title.len()-1]);
    let image_url = String::from(&image_url[1..image_url.len()-1]);
    let _ = message.channel_id.send_message(&ctx.http, |m| m.content(doujin_title)
                                                .embed(|e| e.image(&image_url).description(doujin_tags)));

    Ok(())
}


#[command]
fn bigsmug(ctx: &mut Context, message: &Message) -> CommandResult {
    let url = "https://cdn.discordapp.com/attachments/123165694429888514/520318574343094273/extremelybigsmug4.png";
    let _ = message.channel_id.send_message(&ctx.http, |m| m.embed(|e| e.image(&url)));
    
    Ok(())
}



#[command]
fn chocola2(ctx: &mut Context, message: &Message) -> CommandResult {
    let url = "https://cdn.discordapp.com/attachments/123165694429888514/520318130522685470/extremelybigchocola2.png";
    let _ = message.channel_id.send_message(&ctx.http, |m| m.embed(|e| e.image(&url)));
    
    Ok(())
}


#[command]
fn vanilla2(ctx: &mut Context, message: &Message) -> CommandResult {
    let url = "https://media.discordapp.net/attachments/123165694429888514/556235851747819522/extremelybigvanilla2.png";
    let _ = message.channel_id.send_message(&ctx.http, |m| m.embed(|e| e.image(&url)));
    
    Ok(())
}


#[command]
fn mikasweat(ctx: &mut Context, message: &Message) -> CommandResult {
    let url = "https://cdn.discordapp.com/attachments/123165694429888514/542484609967980565/DisOUS0UwAALPh-.png";
    let _ = message.channel_id.send_message(&ctx.http, |m| m.embed(|e| e.image(&url)));
    
    Ok(())
}
