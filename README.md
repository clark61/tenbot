# Tenbot

## About

A Discord bot that uses a handful of API's users can interact with. Either through text channels or direct messages.

## Screenshots

| ![commands](https://user-images.githubusercontent.com/33743349/159779078-85b8cfb3-b9bc-492b-a797-d725ca07d33b.png) |
| :----------------------------------------------------------------------------------------------------------------: |
|                                                _Command Responses_                                                 |

## Dependencies

-   [Rust](https://www.rust-lang.org/)

There are several API keys that will need to be added to a .env file

-   [Discord](https://discord.com/developers/docs/topics/oauth2)
-   [IEX Cloud](https://iexcloud.io/docs/api/)

```env
DISCORD_TOKEN=yourtoken
GUILD_ID=yourdiscordguildid (Optional, but recommended for development)
IEX_TOKEN=yourtoken
IEX_SANDBOX_TOKEN=yourtoken (Optional, but recommended for development)
```

## Usage

Tenbot can be started by running:

```shell
cargo run
```

in the root of the project.

## Resources

API's that are used:

-   [F1 Standings](https://ergast.com/api/f1)
-   [Financial Data](https://iexcloud.io/docs/api/)
