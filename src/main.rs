// This bot throws a dice on each incoming message.

use teloxide::prelude::*;
use rustrict::CensorStr;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::new("YOUR_BOT_TOKEN");

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        // check whether the message is inappropriate
        let inappropriate: bool = msg.text().unwrap().is_inappropriate();
        if inappropriate {
            bot.send_message(msg.chat.id, "Please don't use inappropriate language!").await?;
            return Ok(());
        }
        Ok(())
    })
    .await;
}
