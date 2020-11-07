use reqwest::get;
use serde_json::Value;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Colour;

#[command]
#[aliases("corgi", "corgipic")]
#[description = "Returns a random picture of a corgi.\nPass no arguments."]
pub async fn corgi_pic(ctx: &Context, msg: &Message) -> CommandResult {
    // Broadcast that the bot is 'typing' to the channel
    msg.channel_id.broadcast_typing(&ctx.http).await?;

    // Retrieve content from request
    let data = reqwest::get("https://dog.ceo/api/breed/corgi/images/random")
        .await?
        .text()
        .await?;

    // Format into json
    let v: Value = serde_json::from_str(&data)?;

    let image = &v["message"].to_string().replace("\"", "");

    // Send an embedded message with the corgi picture to the Discord Channel
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| e.title(":dog:").color(Colour::DARK_PURPLE).image(image));
            m
        })
        .await?;
    Ok(())
}
