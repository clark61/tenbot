use serenity::client::Context;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;

pub async fn ping(ctx: Context, command: ApplicationCommandInteraction) {
    let content = "Hey, I'm alive!".to_string();

    generate_message(ctx, command, content).await;
}

pub async fn generate_message(
    ctx: Context,
    command: ApplicationCommandInteraction,
    content: String,
) {
    if let Err(why) = command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content(content))
        })
        .await
    {
        println!("Cannot respond to slash command: {}", why);
    }
}
