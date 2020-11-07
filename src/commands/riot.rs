use riven::consts::Region;
use riven::RiotApi;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::{Colour, MessageBuilder};
use std::env;

#[command]
#[aliases("lol", "lolstats", "league", "leaguestats")]
#[description = "Returns statistics of a League of Legends summoner.\nPass 1 argument that is the summoner's name.\ntb!league_stats summonerName"]
pub async fn league_stats(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // Broadcast that the bot is 'typing' to the channel
    msg.channel_id.broadcast_typing(&ctx.http).await?;
    
    // Collect summoner name from Discord message
    let summoner_name = args.single::<String>().unwrap();

    let api_key = env::var("RIOT_TOKEN").expect("No riot token in .env");
    let riot_api = RiotApi::with_key(api_key);

    // Save summoner info from Riot response
    let summoner = match riot_api
        .summoner_v4()
        .get_by_summoner_name(Region::NA, &summoner_name)
        .await
        .expect("Failed to get summoner") {
            Some(summoner) => summoner,
            None => {
                if let Err(why) = msg
                    .channel_id
                    .say(&ctx.http, "Could not find a summoner with that name")
                    .await
                {
                println!("Error sending message: {:?}", why);
            }
            panic!("Could not find a summoner with that name")
        }
    };

    // Save champion masteries from the given account name
    let masteries = riot_api
        .champion_mastery_v4()
        .get_all_champion_masteries(Region::NA, &summoner.id)
        .await
        .expect("Failed to get champion masteries");

    // Collect champion name, points, and level to message builder
    let mut content_champion_name = MessageBuilder::new();
    let mut content_champion_points = MessageBuilder::new();
    let mut content_champion_level = MessageBuilder::new();

    for (i, mastery) in masteries[..3].iter().enumerate() {
        content_champion_name.push(format!("{}) {}\n", i + 1, mastery.champion_id.to_string()));
        content_champion_points.push(format!("{}\n", mastery.champion_points));
        content_champion_level.push(format!("({})\n", mastery.champion_level));
    }
    content_champion_name.build();
    content_champion_points.build();
    content_champion_level.build();


    // Format an embeded message to send to Discord channel
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(format!("{}'s Stats", &summoner.name))
                    .color(Colour::DARK_PURPLE)
                    .thumbnail(format!(
                        "http://ddragon.leagueoflegends.com/cdn/10.20.1/img/profileicon/{}.png",
                        &summoner.profile_icon_id
                    ))
                    .description("(WIP)")
                    .field("**-Masteries-**", ":mage:", false)
                    .field("Champion", content_champion_name, true)
                    .field("Points", content_champion_points, true)
                    .field("Level", content_champion_level, true)
            });
            m
        })
        .await?;
    Ok(())
}
