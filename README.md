[![Rust](https://github.com/emkshv/telegram-llm-assistant/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/emkshv/telegram-llm-assistant/actions/workflows/rust.yml)

A minimalistic Telegram assistant with customizable LLM API providers.
It has a very simple installation process: download the binary for your architecture and run it. It will create an Sqlite3 database file, run migrations and start the bot.

Built with:

- [Mobot](https://github.com/0xfe/mobot/) Telegram bot framework
- [Sqlx](https://github.com/launchbadge/sqlx) SQL toolkit
- [Sqlite](https://www.sqlite.org/index.html) self-contained database

### Creating a Telegram bot and obtaining a token

Go to [BotFather](https://telegram.me/BotFather) and enter `/newbot`. Fill in the description and save the token to the `TELEGRAM_TOKEN` environment variable. To define the commands for the autocomplete: enter `/setcommands`, select your bot, and then paste:

```
new - Clear the current context and start a new chat.
get_behavior - Display the current system message that defines the bot's behavior.
set_behavior - Set the new system message for defining the bot's behavior.
get_model - Get the current completion model.
set_model - Set the completion model for your bot.
version - Display the current version.
```

### Running using Docker

Make sure you have [Docker](https://docs.docker.com/get-docker/) & [Docker Compose](https://docs.docker.com/compose/install/). On desktop, you can use [Docker Desktop](https://docker.com/products/docker-desktop/) or [OrbStack](https://orbstack.dev/).

The Docker Compose file expects your environment variables to be loaded:

```
cp .env.example .envrc
# edit .envrc
source .envrc
```

Build the image and run the container:

```
docker build -t telegram-llm-assistant .
docker-compose up
```

### Local development

* Install Rust via [RustUp](https://rustup.rs/)
* `cp .env.example .envrc`
* Edit `.envrc` to set environment variables
* Load environment variables from `.envrc` using [direnv](https://direnv.net/), or `source .envrc`-equivalent in your shell.
* Now you can compile with `cargo build`

### Building from source

* Install Rust via [RustUp](https://rustup.rs/)
* Clone the repository
* Run `cargo build --release`
