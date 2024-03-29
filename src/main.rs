// This bot throws a dice on each incoming message.

use teloxide::{prelude::*, utils::command::BotCommands};
use rustrict::CensorStr;

// define commands
#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "blame a user")]
    Blame(String),
}

async fn handle_commands(
    bot: Bot,
    msg: Message,
    cmd: Command
) -> Result<(), teloxide::RequestError> {
    let text = match cmd {
        Command::Blame(str) => format!("You are to blame, {}!", str),
        Command::Help => Command::descriptions().to_string(),
    };

    bot.send_message(msg.chat.id, text).await?;

    Ok(())
}

async fn handle_message(bot: Bot, msg: Message) -> ResponseResult<()> {
    // get text from the message
    let text = msg.text().unwrap();

    // check for profanity
    let inappropriate: bool = text.is_inappropriate();

    if inappropriate {
        // send a message when inappropriate language is detected
        bot.send_message(msg.chat.id, "Please don't use inappropriate language!").await?;
    } else {
        // compliment the user on their good behavior
        bot.send_message(msg.chat.id, "You are a good person!").await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    // initialize the bot
    pretty_env_logger::init();
    log::info!("Starting profanity bot...");

    let bot = Bot::from_env();

    // set up the dispatcher
    Dispatcher::builder(
        bot,
        Update::filter_message()
            .branch(
                // for commands
                dptree::entry()
                    .filter_command::<Command>()
                    .endpoint(handle_commands))
            .branch(
                // for everything else
                dptree::entry()
                    .endpoint(handle_message))
    )
    .build()
    .dispatch()
    .await;
}
