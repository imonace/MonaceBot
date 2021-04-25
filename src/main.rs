use teloxide::{prelude::*, utils::command::BotCommand};

use std::error::Error;

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "Display this text.")]
    Help,
    #[command(description = "Are you still there?")]
    Ping,
    #[command(description = "handle a username.")]
    Username(String),
    #[command(description = "handle a username and an age.", parse_with = "split")]
    UsernameAndAge { username: String, age: u8 },
}

async fn answer(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help => cx.answer(Command::descriptions()).send().await?,
        Command::Ping => cx.answer("I am still alive.").send().await?,
        Command::Username(username) => cx.answer(format!("Your username is @{}.", username)).await?,
        Command::UsernameAndAge { username, age } => cx.answer(format!("Your username is @{} and age is {}.", username, age)).await?,
    };
    
    Ok(())
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting simple_commands_bot...");

    let bot = Bot::from_env().auto_send();

    let bot_name: String = String::from("MonaceBot");
    teloxide::commands_repl(bot, bot_name, answer).await;
}