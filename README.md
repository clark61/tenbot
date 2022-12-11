# Tenbot

A Discord bot that uses a handful of API's users can interact with through [slash commands](https://support.discord.com/hc/en-us/articles/1500000368501-Slash-Commands-FAQ).

## Dependencies

-   [Rust](https://www.rust-lang.org/)

There are several API keys that will need to be added to a .env file

-   [Discord](https://discord.com/developers/docs/topics/oauth2)
-   [OpenAI](https://beta.openai.com/account/api-keys)

```env
DISCORD_TOKEN=yourtoken
GUILD_ID=yourdiscordguildid (Optional, but recommended for development)
OPENAI_API_KEY=yourkey
```

## Usage

Run the following commands from the root of the project

### Development

```shell
cargo run
```

### Deployment

```shell
cargo build
./target/debug/tenbot
```

## Resources

API's:

-   [F1 Standings](https://ergast.com/api/f1)
-   [OpenAI](https://beta.openai.com/docs/introduction)

Crates:

-   [serenity](https://crates.io/crates/serenity)
-   [tokio](https://crates.io/crates/tokio)
-   [dotenv](https://crates.io/crates/dotenv)
-   [serde_json](https://crates.io/crates/serde_json)
-   [reqwest](https://crates.io/crates/reqwest)
