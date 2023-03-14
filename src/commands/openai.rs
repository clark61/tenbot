use crate::commands::util;
use reqwest::{Client, Error};
use serde_json::json;
use serenity::client::Context;
use serenity::json::Value;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use std::env;

const MODEL: &str = "gpt-3.5-turbo-0301";
const URL: &str = "https://api.openai.com/v1/chat/completions";

async fn create_request(val: Value) -> Result<String, Error> {
    dotenv::dotenv().expect("Failed to load .env file");
    let token = env::var("OPENAI_API_KEY").expect("Expected a token in the environment");

    // Create body. Add period at the end of the prompt to help AI determine the end
    let body = json!({
        "model": MODEL,
        "temperature": 0.3,
        "max_tokens": 2000,
        "top_p": 1.0,
        "frequency_penalty": 0.2,
        "presence_penalty": 0.35,
        "messages": [
            {
                "role": "user",
                "content": format!("{}.", val)
            }
        ]
    });

    let response = Client::new()
        .post(URL)
        .bearer_auth(token)
        .json(&body)
        .send()
        .await;

    let response_text = response.unwrap().text().await;

    response_text
}

async fn parse_text(response: Result<String, Error>) -> String {
    match response {
        Ok(text) => {
            let v: Value = serde_json::from_str(&text).unwrap();
            let output = &v["choices"][0]["message"]["content"];

            output
                .to_string()
                .replace("\\n\\n", " ")
                .replace("\\n", " ")
                .replace("\\\"", "")
                .replace("\"", "")
                .trim()
                .to_string()
        }
        Err(_) => "Did not receive a response from Open Ai :(".to_string(),
    }
}

pub async fn text_prompt(ctx: Context, command: ApplicationCommandInteraction) {
    let value = &command
        .data
        .options
        .get(0)
        .expect("Expected user to select option")
        .value;

    match value.to_owned() {
        Some(val) => {
            // Responses from OpenAI can take more than 3 seconds to be generated.
            // To avoid timing out the bot, generate an initial message and then edit the message
            // later when the AI response has been retrieved.
            util::generate_message(
                ctx.to_owned(),
                command.to_owned(),
                format!("```{}```", "Please wait... Fetching response..."),
            )
            .await;

            let response = create_request(val).await;
            let text = parse_text(response).await;

            // Edit the initial message with AI response
            util::edit_generated_message(ctx, command, format!("```{}```", text)).await
        }
        None => util::generate_message(ctx, command, "Could not pass text to AI".to_string()).await,
    }
}
