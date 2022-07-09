use crate::commands::util;
use serde_json::Value;
use serenity::builder::{CreateEmbed, CreateEmbedFooter};
use serenity::client::Context;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::utils::{Colour, MessageBuilder};

struct Standings {
    drivers: Option<MessageBuilder>,
    constructors: Option<MessageBuilder>,
    points: Option<MessageBuilder>,
}

struct SeasonCalendar {
    season_year: String,
    rounds: MessageBuilder,
    race_names: MessageBuilder,
    race_dates: MessageBuilder,
}

/// Collects json response from Ergast API call to get constructor standings
async fn get_constructor_standings() -> Standings {
    let url = "https://ergast.com/api/f1/current/constructorStandings.json";
    let mut constructor_names = MessageBuilder::new();
    let mut constructor_points = MessageBuilder::new();
    let standings: Standings;

    let data = reqwest::get(url).await.unwrap().text().await.unwrap();
    let v: Value = serde_json::from_str(&data).unwrap();
    let info = &v["MRData"]["StandingsTable"]["StandingsLists"][0]["ConstructorStandings"];

    for i in 0..10 {
        let name = info[i]["Constructor"]["name"].to_string().replace("\"", "");
        let points = info[i]["points"].to_string().replace("\"", "");

        constructor_names.push(format!("{}\n", name));
        constructor_points.push(format!("{}\n", points));
    }

    standings = Standings {
        drivers: None,
        constructors: Some(constructor_names),
        points: Some(constructor_points),
    };

    standings
}

/// Collects json response from Ergast API call to get driver standings
async fn get_driver_standings() -> Standings {
    let url = "https://ergast.com/api/f1/current/driverStandings.json";
    let mut driver_names = MessageBuilder::new();
    let mut driver_constructors = MessageBuilder::new();
    let mut driver_points = MessageBuilder::new();
    let standings: Standings;

    let data = reqwest::get(url).await.unwrap().text().await.unwrap();
    let v: Value = serde_json::from_str(&data).unwrap();
    let info = &v["MRData"]["StandingsTable"]["StandingsLists"][0]["DriverStandings"];

    for i in 0..20 {
        let points = info[i]["points"].to_string().replace("\"", "");
        let constructor = info[i]["Constructors"][0]["name"]
            .to_string()
            .replace("\"", "");
        let name = info[i]["Driver"]["familyName"]
            .to_string()
            .replace("\"", "");

        driver_names.push(format!("{}\n", name));
        driver_constructors.push(format!("{}\n", constructor));
        driver_points.push(format!("{}\n", points));
    }

    standings = Standings {
        drivers: Some(driver_names),
        constructors: Some(driver_constructors),
        points: Some(driver_points),
    };

    standings
}

/// Collects json response from Ergast API call to get the season's calendar
async fn get_season_calendar() -> SeasonCalendar {
    let url = "https://ergast.com/api/f1/current.json";
    let mut season_rounds = MessageBuilder::new();
    let mut race_names = MessageBuilder::new();
    let mut race_dates = MessageBuilder::new();
    let seasonCalendar: SeasonCalendar;

    let data = reqwest::get(url).await.unwrap().text().await.unwrap();
    let v: Value = serde_json::from_str(&data).unwrap();
    let info = &v["MRData"]["RaceTable"]["Races"];
    let total_rounds = get_total_rounds().await;
    let current_season = get_season_year(&info).to_string();

    for i in 0..total_rounds as usize {
        let rounds = info[i]["round"].to_string().replace("\"", "");
        let name = info[i]["Circuit"]["Location"]["country"]
            .to_string()
            .replace("\"", "");
        let date = info[i]["date"].to_string().replace("\"", "");

        race_dates.push(format!("{}\n", date));
        race_names.push(format!("{}\n", name));
        season_rounds.push(format!("{}\n", rounds));
    }

    seasonCalendar = SeasonCalendar {
        season_year: current_season,
        rounds: season_rounds,
        race_names: race_names,
        race_dates: race_dates,
    };

    seasonCalendar
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

/// Retrieves F1 constructor standings and outputs results through an embedded message
pub async fn constructor_standings(ctx: Context, command: ApplicationCommandInteraction) {
    // Collect constructor info
    let standings = get_constructor_standings().await;

    // Format embedded message
    let mut embed = CreateEmbed::default();
    embed.title("Current Constructor Standings");
    embed.colour(Colour::DARK_RED);
    embed.thumbnail("https://1000logos.net/wp-content/uploads/2020/02/F1-Logo-500x281.png");
    embed.field("Constructor", standings.constructors.unwrap(), true);
    embed.field("Points", standings.points.unwrap(), true);

    // Attempt to send response
    util::generate_embed_message(ctx, command, embed).await
}

/// Retrieves F1 driver standings and outputs results through an embedded message
pub async fn driver_standings(ctx: Context, command: ApplicationCommandInteraction) {
    // Collect driver info
    let standings = get_driver_standings().await;

    // Format embedded message
    let mut embed = CreateEmbed::default();
    embed.title("Current Driver Standings");
    embed.colour(Colour::DARK_RED);
    embed.thumbnail("https://1000logos.net/wp-content/uploads/2020/02/F1-Logo-500x281.png");
    embed.field("Name", standings.drivers.unwrap(), true);
    embed.field("Constructor", standings.constructors.unwrap(), true);
    embed.field("Points", standings.points.unwrap(), true);

    // Attempt to send response
    util::generate_embed_message(ctx, command, embed).await
}

pub async fn season_calendar(ctx: Context, command: ApplicationCommandInteraction) {
    // Collect season info
    let calendar = get_season_calendar().await;

    // Create footer
    let mut footer = CreateEmbedFooter::default();
    footer.text("https://f1calendar.com/");
    footer.icon_url("https://raw.githubusercontent.com/sportstimes/f1/9cdaa32dba300930b944bc739517063147cae5b2/_public/f1/mstile-70x70.png");

    // Format embedded message
    let mut embed = CreateEmbed::default();
    embed.title(format!("{} Season Calendar", calendar.season_year));
    embed.colour(Colour::DARK_RED);
    embed.thumbnail("https://1000logos.net/wp-content/uploads/2020/02/F1-Logo-500x281.png");
    embed.field("Round", calendar.rounds, true);
    embed.field("GP", calendar.race_names, true);
    embed.field("Date", calendar.race_dates, true);
    embed.footer(|footer| {
        footer
        .text("https://f1calendar.com/")
        .icon_url("https://raw.githubusercontent.com/sportstimes/f1/9cdaa32dba300930b944bc739517063147cae5b2/_public/f1/mstile-70x70.png")
    });

    // Attempt to send response
    util::generate_embed_message(ctx, command, embed).await
}

// https://ergast.com/api/f1/current/last/results.json
