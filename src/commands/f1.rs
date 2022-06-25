use crate::commands::util;
use serde_json::Value;
use serenity::builder::{CreateEmbed, CreateEmbedFooter};
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

    // Get names for the drivers
    for i in 0..20 {
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

    // Get the driver's constructor
    for i in 0..20 {
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

    // Get the driver's points
    for i in 0..20 {
        let points = info[i]["points"].to_string().replace("\"", "");
        driver_points.push(format!("{}\n", points));
    }
    driver_points
}

/// Collects json response from Ergast API call to get the season's calendar
async fn get_season_calendar() -> Value {
    let url = "https://ergast.com/api/f1/current.json";

    let data = reqwest::get(url).await.unwrap().text().await.unwrap();
    let v: Value = serde_json::from_str(&data).unwrap();
    v["MRData"]["RaceTable"]["Races"].clone()
}

/// Return the total amount of races for the current season
async fn get_total_rounds() -> u8 {
    let url = "https://ergast.com/api/f1/current.json";

    let data = reqwest::get(url).await.unwrap().text().await.unwrap();
    let v: Value = serde_json::from_str(&data).unwrap();
    let total: u8 = v["MRData"]["total"].as_str().unwrap().parse().unwrap();
    total
}

/// Returns the most recent season's year
fn get_season_year(info: &Value) -> &str {
    info[0]["season"].as_str().unwrap()
}

/// Returns a column of each race's round #
fn get_season_rounds(info: &Value, total_rounds: usize) -> MessageBuilder {
    let mut season_rounds = MessageBuilder::new();

    // Get the round # for each race
    for i in 0..total_rounds as usize {
        let rounds = info[i]["round"].to_string().replace("\"", "");
        season_rounds.push(format!("{}\n", rounds));
    }
    season_rounds
}

/// Returns a column of each race's name
fn get_race_names(info: &Value, total_rounds: usize) -> MessageBuilder {
    let mut race_names = MessageBuilder::new();

    // Collect all race names
    for i in 0..total_rounds as usize {
        let name = info[i]["Circuit"]["Location"]["country"]
            .to_string()
            .replace("\"", "");
        race_names.push(format!("{}\n", name));
    }
    race_names
}

/// Returns a column of each race's GP date
fn get_race_dates(info: &Value, total_rounds: usize) -> MessageBuilder {
    let mut race_dates = MessageBuilder::new();

    // Collect all race names
    for i in 0..total_rounds as usize {
        let date = info[i]["date"].to_string().replace("\"", "");
        race_dates.push(format!("{}\n", date));
    }
    race_dates
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
    embed.colour(Colour::DARK_RED);
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
    embed.colour(Colour::DARK_RED);
    embed.thumbnail("https://1000logos.net/wp-content/uploads/2020/02/F1-Logo-500x281.png");
    embed.field("Name", driver_names, true);
    embed.field("Constructor", driver_constructors, true);
    embed.field("Points", driver_points, true);

    // Attempt to send response
    util::generate_embed_message(ctx, command, embed).await
}

pub async fn season_calendar(ctx: Context, command: ApplicationCommandInteraction) {
    // Collect season info
    let info = get_season_calendar().await;
    let total_rounds = get_total_rounds().await;
    let current_season = get_season_year(&info);
    let rounds = get_season_rounds(&info, total_rounds as usize);
    let race_names = get_race_names(&info, total_rounds as usize);
    let race_dates = get_race_dates(&info, total_rounds as usize);

    // Create footer
    let mut footer = CreateEmbedFooter::default();
    footer.text("https://f1calendar.com/");
    footer.icon_url("https://raw.githubusercontent.com/sportstimes/f1/9cdaa32dba300930b944bc739517063147cae5b2/_public/f1/mstile-70x70.png");

    // Format embedded message
    let mut embed = CreateEmbed::default();
    embed.title(format!("{} Season Calendar", current_season));
    embed.colour(Colour::DARK_RED);
    embed.thumbnail("https://1000logos.net/wp-content/uploads/2020/02/F1-Logo-500x281.png");
    embed.field("Round", rounds, true);
    embed.field("GP", race_names, true);
    embed.field("Date", race_dates, true);
    embed.footer(|footer| {
        footer
        .text("https://f1calendar.com/")
        .icon_url("https://raw.githubusercontent.com/sportstimes/f1/9cdaa32dba300930b944bc739517063147cae5b2/_public/f1/mstile-70x70.png")
    });

    // Attempt to send response
    util::generate_embed_message(ctx, command, embed).await
}
