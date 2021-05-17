use std::error::Error;
use teloxide::{prelude::*, types::ParseMode, utils::command::BotCommand};

mod suse;

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "are you still there?")]
    Ping,
    #[command(description = "query a openSUSE pkg version.")]
    Pkg(String),
}

async fn answer(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help => cx.answer(Command::descriptions()).send().await?,
        Command::Ping => cx.answer("I am still alive.").send().await?,
        Command::Pkg(pkgname) => {
            cx.answer(suse::get_pkg(pkgname).await)
                .parse_mode(ParseMode::Html)
                .send()
                .await?
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting MonaceBot...");

    let bot = Bot::from_env().auto_send();

    let bot_name = String::from("monacebot");
    teloxide::commands_repl(bot, bot_name, answer).await;
}
