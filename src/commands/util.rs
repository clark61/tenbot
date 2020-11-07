use rand::Rng;
use serenity::framework::standard::{macros::command, Args, CommandResult, Delimiter};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Colour;

#[command]
#[aliases("user_info", "userinfo")]
#[description = "Returns info about the user that called this command.\nPass no arguments."]
pub async fn user_info(ctx: &Context, msg: &Message) -> CommandResult {
    // Collect user info
    let tag = msg.author.tag();
    let avatar = match msg.author.avatar_url() {
        Some(avatar) => avatar,
        None => msg.author.default_avatar_url(),
    };
    let creation_date = msg.author.created_at();
    let user_id = msg.author.id;

    // Format and send an embedded message
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(tag)
                    .color(Colour::DARK_PURPLE)
                    .thumbnail(avatar)
                    .field("Creation Date", creation_date, false)
                    .field("User ID", user_id, false)
            });
            m
        })
        .await?;

    Ok(())
}

#[command]
#[aliases("server_info", "serverinfo")]
#[description = "Returns info about the Discord Server.\nPass no arguments."]
pub async fn server_info(ctx: &Context, msg: &Message) -> CommandResult {
    // Collect server info if possible
    if let Some(guild) = msg.guild_id {
        let creation_date = guild.created_at();
        let server_info = guild
            .to_partial_guild(&ctx.http)
            .await
            .expect("Couldnt get server info");
        let server_id = &server_info.id;
        let server_name = &server_info.name;
        let server_region = &server_info.region;
        let server_icon = match server_info.icon_url() {
            Some(icon) => icon,
            None => {
                let mut s = String::new();
                s.push_str("https://discord.com/assets/e7a3b51fdac2aa5ec71975d257d5c405.png");
                s
            }
        };

        let server_desc = match server_info.description {
            Some(desc) => desc,
            None => {
                let mut s = String::new();
                s.push_str("N/A");
                s
            }
        };

        // Format and send an embedded message
        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title(server_name)
                        .color(Colour::DARK_PURPLE)
                        .thumbnail(server_icon)
                        .field("Server Description", server_desc, false)
                        .field("Creation Date", creation_date, false)
                        .field("Server ID", server_id, false)
                        .field("Server Region", server_region, false)
                });
                m
            })
            .await?;
    }
    Ok(())
}

#[command]
#[aliases("pick")]
#[description = "Chooses 1 option from a given list of choices.\nPass 1 argument that is a list of options seperated by a comma.\ntb!choose option1, option2, option3"]
pub async fn choose(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    // Collect choices and store in a vector
    let args = Args::new(args.message(), &[Delimiter::Single(',')]);
    let choices = args.raw_quoted().collect::<Vec<&str>>();

    // Generate a random index to access from the vector
    let rand_choice = rand::thread_rng().gen_range(0, choices.len());
    let choice = choices[rand_choice].to_string().replace("\"", "");

    // Send the choice back to the channel
    msg.channel_id
        .say(&ctx.http, format!("I choose {}!", choice))
        .await?;
    Ok(())
}

#[command]
#[aliases("flip", "coinflip", "flipcoin")]
#[description = "Flips a coin and returns 'heads' or 'tails.'\nPass no arguments."]
pub async fn coin_flip(ctx: &Context, msg: &Message) -> CommandResult {
    let choice = rand::thread_rng().gen_range(0, 2);

    if choice == 0 {
        msg.channel_id.say(&ctx.http, format!("Heads!")).await?;
    } else {
        msg.channel_id.say(&ctx.http, format!("Tails!")).await?;
    }

    Ok(())
}
