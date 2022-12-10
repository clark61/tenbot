mod commands;

use std::env;

use serenity::async_trait;
use serenity::model::application::command::{Command, CommandOptionType};
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::{Activity, Ready};
use serenity::model::user::OnlineStatus;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!(
                "Received command: {:#?} from user: {:#?}",
                command.data.name, command.user.name
            );

            match command.data.name.as_str() {
                "ping" => commands::util::ping(ctx, command).await,
                "ai" => {
                    let option: &str = command
                        .data
                        .options
                        .get(0)
                        .expect("Expected user to select option")
                        .name
                        .as_ref();
                    match option {
                        "prompt" => commands::openai::text_prompt(ctx, command).await,
                        _ => {
                            commands::util::generate_message(
                                ctx,
                                command,
                                "Invalid option".to_string(),
                            )
                            .await
                        }
                    }
                }
                "f1" => {
                    let option: &str = command
                        .data
                        .options
                        .get(0)
                        .expect("Expected user to select option")
                        .name
                        .as_ref();
                    match option {
                        "constructors" => commands::f1::constructor_standings(ctx, command).await,
                        "drivers" => commands::f1::driver_standings(ctx, command).await,
                        "calendar" => commands::f1::season_calendar(ctx, command).await,
                        "recent_race_results" => {
                            commands::f1::recent_race_results(ctx, command).await
                        }
                        _ => {
                            commands::util::generate_message(
                                ctx,
                                command,
                                "Invalid option".to_string(),
                            )
                            .await
                        }
                    }
                }
                _ => {
                    commands::util::generate_message(
                        ctx,
                        command,
                        "Not implemented :(".to_string(),
                    )
                    .await;
                }
            };
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        ctx.set_presence(
            Some(Activity::playing("| Accepting slash commands!")),
            OnlineStatus::Online,
        )
        .await;

        // create global commands
        let _ = Command::create_global_application_command(&ctx.http, |command| {
            command
                .name("ping")
                .description("A ping command to verify if the bot is accepting comands")
        })
        .await;

        let _ = Command::create_global_application_command(&ctx.http, |command| {
            command
                .name("f1")
                .description("Get the current F1 standings and calendar")
                .create_option(|option| {
                    option
                        .name("constructors")
                        .description("Get current constructor standings")
                        .kind(CommandOptionType::SubCommand)
                })
                .create_option(|option| {
                    option
                        .name("drivers")
                        .description("Get current driver standings")
                        .kind(CommandOptionType::SubCommand)
                })
                .create_option(|option| {
                    option
                        .name("calendar")
                        .description("Get the season's calendar")
                        .kind(CommandOptionType::SubCommand)
                })
                .create_option(|option| {
                    option
                        .name("recent_race_results")
                        .description("Get the results from the most recent Grand Prix")
                        .kind(CommandOptionType::SubCommand)
                })
        })
        .await;

        let _ = Command::create_global_application_command(&ctx.http, |command| {
            command
                .name("ai")
                .description("Interact with an Open AI model")
                .create_option(|option| {
                    option
                        .name("prompt")
                        .description("The text sent to OpenAI")
                        .kind(CommandOptionType::String)
                })
        })
        .await;
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with Discord bot token in the environment.
    dotenv::dotenv().expect("Failed to load .env file");
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Build client.
    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
