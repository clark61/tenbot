use reqwest::get;
use serde_json::Value;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::{Colour, MessageBuilder};

#[command]
#[aliases("constructorstandings", "constructors")]
#[description = "Returns current F1 constructor standings.\nPass no arguments."]
pub async fn constructor_standings(ctx: &Context, msg: &Message) -> CommandResult {
    // Broadcast that the bot is 'typing' to the channel
    msg.channel_id.broadcast_typing(&ctx.http).await?;

    // Collect constructor standing information from json
    let data = reqwest::get("https://ergast.com/api/f1/current/constructorStandings.json")
        .await?
        .text()
        .await?;
    let v: Value = serde_json::from_str(&data)?;
    let info = &v["MRData"]["StandingsTable"]["StandingsLists"][0]["ConstructorStandings"];
    // Format message builders
    let mut constructor_name = MessageBuilder::new();
    let mut constructor_points = MessageBuilder::new();

    for i in 0..10 {
        let name = info[i]["Constructor"]["name"].to_string().replace("\"", "");
        constructor_name.push(format!("{}\n", name));

        let points = info[i]["points"].to_string().replace("\"", "");
        constructor_points.push(format!("{}\n", points));
    }

    // Send embedded message
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(format!("Current Constructor Standings"))
                    .color(Colour::DARK_PURPLE)
                    .thumbnail(
                        "https://1000logos.net/wp-content/uploads/2020/02/F1-Logo-500x281.png",
                    )
                    .field("Constructor", constructor_name, true)
                    .field("Points", constructor_points, true)
            });
            m
        })
        .await?;

    Ok(())
}

#[command]
#[aliases("driverstandings", "drivers")]
#[description = "Returns the current F1 driver standings.\nPass no arguments."]
pub async fn driver_standings(ctx: &Context, msg: &Message) -> CommandResult {
    // Broadcast that the bot is 'typing' to the channel
    msg.channel_id.broadcast_typing(&ctx.http).await?;

    // Collect constructor standing information from json
    let data = reqwest::get("https://ergast.com/api/f1/current/driverStandings.json")
        .await?
        .text()
        .await?;
    let v: Value = serde_json::from_str(&data)?;
    let info = &v["MRData"]["StandingsTable"]["StandingsLists"][0]["DriverStandings"];

    // Format message builders
    let mut driver_name = MessageBuilder::new();
    let mut driver_constructor = MessageBuilder::new();
    let mut driver_points = MessageBuilder::new();

    // TODO: Decide if all drivers should be included or just top 10
    for i in 0..10 {
        let last_name = info[i]["Driver"]["familyName"]
            .to_string()
            .replace("\"", "");
        driver_name.push(format!("{}\n", last_name));

        let constructor = info[i]["Constructors"][0]["name"]
            .to_string()
            .replace("\"", "");
        driver_constructor.push(format!("{}\n", constructor));

        let points = info[i]["points"].to_string().replace("\"", "");
        driver_points.push(format!("{}\n", points));
    }

    // Send embedded message
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(format!("Current Driver Standings"))
                    .color(Colour::DARK_PURPLE)
                    .thumbnail(
                        "https://1000logos.net/wp-content/uploads/2020/02/F1-Logo-500x281.png",
                    )
                    .field("Name", driver_name, true)
                    .field("Constructor", driver_constructor, true)
                    .field("Points", driver_points, true)
            });
            m
        })
        .await?;

    Ok(())
}
