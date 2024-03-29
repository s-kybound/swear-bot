use teloxide::{prelude::*, utils::command::BotCommands};
use rustrict::CensorStr;

// define commands
#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text")]
    Help,
    #[command(description = "about the bot")]
    About,
    #[command(description = "blame a user")]
    Blame(String),
}

fn is_username(str: &str) -> bool {
    // check if the string is a valid username,
    // which is the case if it starts with @ and contains only alphanumeric characters and underscores
    str.starts_with('@') && str.chars().skip(1).all(|c| c.is_alphanumeric() || c == '_')
}

async fn handle_commands(
    bot: Bot,
    msg: Message,
    cmd: Command
) -> Result<(), teloxide::RequestError> {
    let text = match cmd {
        Command::Blame(str) => {
            // ensure str is not empty
            if str.trim().is_empty() {
                "USAGE: /blame <username>".to_string()
            } else if !is_username(&str) {
                "<username> should start with @".to_string()
            } else {
                // get the user who blamed someone
                let blamer = msg.from().unwrap().username.clone().unwrap_or("someone".to_string());
                format!("@{} says that {} said a BAD WORD!", blamer, str)
            }
        },
        Command::Help => Command::descriptions().to_string(),
        Command::About => "---kyriel-swear-bot v0.0.1-beta-prelease4-testing---\n \
        Repository: https://github.com/s-kybound/swear-bot\n \
        This bot detects inappropriate language in group chats and shames the user who used it.\n \
        TODO: Automatic paylah payment request on swear, swear leadership boards, statistics on most commonly used swear words per user"
        .to_string(),
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

        // get the naughty user
        let naughty_user = msg.from().unwrap().username.clone().unwrap_or("someone".to_string());

        // flame them
        bot.send_message(msg.chat.id, format!("@{} said a BAD WORD!", naughty_user)).await?;
    }

    Ok(())
}

async fn handle_test_message(bot: Bot, msg: Message) -> ResponseResult<()> {
    // get text from the message
    let text = msg.text().unwrap();

    // check for profanity
    let inappropriate: bool = text.is_inappropriate();

    if inappropriate {
        // send a message when inappropriate language is detected
        bot.send_message(msg.chat.id, "DEBUG: Inappropriate language detected").await?;
    } else {
        // compliment the user on their good behavior
        bot.send_message(msg.chat.id, "DEBUG: No inappropriate language").await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    // before starting, we add singlish swear words to the censor


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
                // for case where context is not in group
                dptree::filter(|msg: Message| !(msg.chat.is_group() || msg.chat.is_supergroup()))
                    .endpoint(handle_test_message))
            .branch(
                // for commands
                dptree::entry()
                    .filter_command::<Command>()
                    .endpoint(handle_commands))
            .branch(
                // for everything else
                dptree::filter(|_msg: Message| true)
                    .endpoint(handle_message))
    )
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}
