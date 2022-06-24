mod commands;

use std::env;

use serenity::async_trait;
use serenity::model::application::command::{Command, CommandOptionType};
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            match command.data.name.as_str() {
                "ping" => commands::util::ping(ctx, command).await,
                "f1" => {
                    let option: &str = command
                        .data
                        .options
                        .get(0)
                        .expect("Expected user to select option")
                        .name
                        .as_ref();
                    match option {
                        "constructor" => commands::f1::constructor_standings(ctx, command).await,
                        "driver" => commands::f1::driver_standings(ctx, command).await,
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

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands.create_application_command(|command| {
                command
                    .name("f1")
                    .description("Get the current F1 driver or constructor standings")
                    .create_option(|option| {
                        option
                            .name("constructor")
                            .description("Get current constructor standings")
                            .kind(CommandOptionType::SubCommand)
                    })
                    .create_option(|option| {
                        option
                            .name("driver")
                            .description("Get current driver standings")
                            .kind(CommandOptionType::SubCommand)
                    })
            })
        })
        .await;

        println!(
            "I now have the following guild slash commands: {:#?}",
            commands
        );

        // create global commands
        let _ = Command::create_global_application_command(&ctx.http, |command| {
            command
                .name("ping")
                .description("A ping command to verify if the bot is accepting comands")
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
