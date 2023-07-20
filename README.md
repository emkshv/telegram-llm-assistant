A minimalistic, zero-dependency, ChatGPT-powered Telegram assistant written in Rust.
It has a very simple installation process: download the binary for your architecture and run it. It will create an sqlite3 database file, run migrations and start the bot.

Built with:

- [Mobot](https://github.com/0xfe/mobot/) Telegram bot framework
- [Sqlx](https://github.com/launchbadge/sqlx) SQL toolkit
- [Sqlite](https://www.sqlite.org/index.html) self-contained database

### Deployment using Docker

Build the image

```
docker build -t helpful-assistant .
```

### Local development

1. Install Rust via [RustUp](https://rustup.rs/)
2. Install Sqlite3 (e.g., `sudo apt install sqlite3` / `brew install sqlite3`)
3. `cp .env.example .envrc`
4. Edit `.envrc` to set environment variables
5. Load environment variables from `.envrc` using [direnv](https://direnv.net/), or `source .envrc`-equivalent in your shell.
4. Run `sqlite3 $(basename $BOT_SQLITE_DATABASE_URL) < schema.sql` where `$BOT_SQLITE_DATABASE_URL` is your url with the local path to the Sqlite DB file.
6. Now you can compile with `cargo build`
