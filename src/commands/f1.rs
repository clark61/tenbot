use crate::commands::util;
use serde_json::Value;
use serenity::builder::CreateEmbed;
use serenity::client::Context;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::utils::{Colour, MessageBuilder};

/// Collects json response from Ergast API call to get constructor standings
async fn get_constructor_standings() -> Value {
    let url = "https://ergast.com/api/f1/current/constructorStandings.json";

    let data = reqwest::get(url).await.unwrap().text().await.unwrap();
    let v: Value = serde_json::from_str(&data).unwrap();
    v["MRData"]["StandingsTable"]["StandingsLists"][0]["ConstructorStandings"].clone()
}

/// Collects json response from Ergast API call to get driver standings
async fn get_driver_standings() -> Value {
    let url = "https://ergast.com/api/f1/current/driverStandings.json";

    let data = reqwest::get(url).await.unwrap().text().await.unwrap();
    let v: Value = serde_json::from_str(&data).unwrap();
    v["MRData"]["StandingsTable"]["StandingsLists"][0]["DriverStandings"].clone()
}

/// Returns a column of F1 constructor names
fn get_constructor_names(info: &Value) -> MessageBuilder {
    let mut constructor_names = MessageBuilder::new();

    // Get names from the constructors
    for i in 0..10 {
        let name = info[i]["Constructor"]["name"].to_string().replace("\"", "");
        constructor_names.push(format!("{}\n", name));
    }
    constructor_names
}

/// Returns a column of F1 constructor points
fn get_constructor_points(info: &Value) -> MessageBuilder {
    let mut constructor_points = MessageBuilder::new();

    // Get points from the constructors
    for i in 0..10 {
        let points = info[i]["points"].to_string().replace("\"", "");
        constructor_points.push(format!("{}\n", points));
    }
    constructor_points
}

/// Returns a column of F1 driver names
fn get_driver_names(info: &Value) -> MessageBuilder {
    let mut driver_names = MessageBuilder::new();

    // Get names from the top 10 drivers
    for i in 0..10 {
        let name = info[i]["Driver"]["familyName"]
            .to_string()
            .replace("\"", "");
        driver_names.push(format!("{}\n", name));
    }
    driver_names
}

/// Returns a column of constructors for each driver
fn get_driver_constructors(info: &Value) -> MessageBuilder {
    let mut driver_constructors = MessageBuilder::new();

    // Get the driver's constructor for the top 10 drivers
    for i in 0..10 {
        let constructor = info[i]["Constructors"][0]["name"]
            .to_string()
            .replace("\"", "");
        driver_constructors.push(format!("{}\n", constructor));
    }
    driver_constructors
}

/// Returns a column of points for each driver
fn get_driver_points(info: &Value) -> MessageBuilder {
    let mut driver_points = MessageBuilder::new();

    // Get the driver's points for the top 10 drivers
    for i in 0..10 {
        let points = info[i]["points"].to_string().replace("\"", "");
        driver_points.push(format!("{}\n", points));
    }
    driver_points
}

/// Retrieves F1 constructor standings and outputs results through an embedded message
pub async fn constructor_standings(ctx: Context, command: ApplicationCommandInteraction) {
    // Collect constructor info
    let info = get_constructor_standings().await;
    let constructor_names = get_constructor_names(&info);
    let constructor_points = get_constructor_points(&info);

    // Format embedded message
    let mut embed = CreateEmbed::default();
    embed.title("Current Constructor Standings");
    embed.colour(Colour::DARK_PURPLE);
    embed.thumbnail("https://1000logos.net/wp-content/uploads/2020/02/F1-Logo-500x281.png");
    embed.field("Constructor", constructor_names, true);
    embed.field("Points", constructor_points, true);

    // Attempt to send response
    util::generate_embed_message(ctx, command, embed).await
}

/// Retrieves F1 driver standings and outputs results through an embedded message
pub async fn driver_standings(ctx: Context, command: ApplicationCommandInteraction) {
    // Collect driver info
    let info = get_driver_standings().await;
    let driver_names = get_driver_names(&info);
    let driver_constructors = get_driver_constructors(&info);
    let driver_points = get_driver_points(&info);

    // Format embedded message
    let mut embed = CreateEmbed::default();
    embed.title("Current Driver Standings");
    embed.colour(Colour::DARK_PURPLE);
    embed.thumbnail("https://1000logos.net/wp-content/uploads/2020/02/F1-Logo-500x281.png");
    embed.field("Name", driver_names, true);
    embed.field("Constructor", driver_constructors, true);
    embed.field("Points", driver_points, true);

    // Attempt to send response
    util::generate_embed_message(ctx, command, embed).await
}
