use std::sync::{Arc, Mutex};
use rusqlite::{params, Connection, Result};
use teloxide::{
    dispatching::dialogue::GetChatId, prelude::*, utils::command::BotCommands
};
use rustrict::CensorStr;

mod messages;

const DATABASE: &str = "./src/database.db";

// define commands
#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text")]
    Help,
    #[command(description = "about the bot")]
    About,
    #[command(description = "blame a user for swearing")]
    Blame(String),
    #[command(description = "show the swear leaderboard")]
    Leaderboard,
    #[command(description = "show the swear statistics for a user in the chat")]
    Expose(String),
    #[command(description = "make the bot shut up for the meantime")]
    Stfu,
    #[command(description = "make the bot noisy again")]
    Wakeup
}

fn is_username(str: &str) -> bool {
    // check if the string is a valid username,
    // which is the case if it starts with @ and contains only alphanumeric characters and underscores
    str.starts_with('@') && str.chars().skip(1).all(|c| c.is_alphanumeric() || c == '_')
}

async fn handle_commands(
    bot: Bot,
    msg: Message,
    db: Arc<Mutex<Connection>>,
    silent: Arc<Mutex<bool>>,
    cmd: Command,
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
                let blamer = msg.from().unwrap().username.clone().unwrap_or(msg.from().unwrap().full_name());
                let naughty_user = str.trim_start_matches('@').to_string();
                let answer: String;
                // check if the user exists in the group
                {
                    let conn = db.lock().unwrap();
                    let ChatId(chat_id) = msg.chat.id;
                    let mut stmt = conn.prepare("SELECT swear_count FROM swearers WHERE username = ? AND chat_id = ?").unwrap();
                    let mut rows = stmt.query(params![naughty_user, chat_id]).unwrap();
                    if let Some(row) = rows.next().unwrap() {
                        // then increment the swear count
                        let swear_count: i32 = row.get(0).unwrap();
                        conn.execute("UPDATE swearers SET swear_count = ? WHERE username = ? AND chat_id = ?", params![swear_count + 1, naughty_user, chat_id]).unwrap();
                        answer = messages::make_blame_swear_message(blamer, naughty_user);
                    } else {
                        answer = messages::blamer_is_innocent_message(blamer, naughty_user);
                    }
                }
                answer
            }
        },
        Command::Help => Command::descriptions().to_string(),
        Command::About => "---kyriel-swear-bot v0.0.2-beta-prerelease-4-testing---\n \
        Repository: https://github.com/s-kybound/swear-bot\n \
        This bot detects inappropriate language in group chats and shames the user who used it.\n \
        TODO: automatic paylah payment request on swear, swear leadership boards, statistics on most commonly used swear words per user"
        .to_string(),
        Command::Leaderboard => {
            {
                let conn = db.lock().unwrap();
                let ChatId(chat_id) = msg.chat.id;
                let mut stmt = conn.prepare("SELECT username, swear_count FROM swearers WHERE chat_id = ? ORDER BY swear_count DESC LIMIT 15").unwrap();
                let mut rows = stmt.query(params![chat_id]).unwrap();
                let mut text = "NAUGHTIEST texters:\n".to_string();
                let mut i = 1;
                while let Some(row) = rows.next().unwrap() {
                    let user: String = row.get(0).unwrap();
                    let swear_count: i32 = row.get(1).unwrap();
                    text.push_str(&format!("{}. @{}: {}\n", i, user, swear_count));
                    i += 1;
                }
                // get the worst offender's swear count again
                let mut stmt = conn.prepare("SELECT username, swear_count FROM swearers WHERE chat_id = ? ORDER BY swear_count DESC LIMIT 1").unwrap();
                let mut rows = stmt.query(params![chat_id]).unwrap();
                if let Some(row) = rows.next().unwrap() {
                    let user: String = row.get(0).unwrap();
                    let swear_count: i32 = row.get(1).unwrap();
                    text.push_str(&format!("\nWORST OFFENDER: @{} with {} swears\n", user, swear_count));

                    let actual_cost = swear_count as f64 * 0.1;
                    text.push_str(format!("total cost: ${:.2}\n", actual_cost).as_str());
                    text.push_str("FUN FACT:\n");
                    text.push_str(messages::cost_fun_fact(actual_cost).as_str());
                }
                text
            }
        },
        Command::Expose(user) => {
            // ensure user is not empty
            if user.trim().is_empty() {
                "USAGE: /expose <username>".to_string()
            } else if !is_username(&user) {
                "<username> should start with @".to_string()
            } else {
                // get the statistics for the user
                {
                    let conn = db.lock().unwrap();
                    let ChatId(chat_id) = msg.chat.id;
                    let username = user.trim_start_matches('@');
                    let mut stmt = conn.prepare("SELECT swear_count FROM swearers WHERE username = ? AND chat_id = ?").unwrap();
                    let mut rows = stmt.query(params![username, chat_id]).unwrap();
                    if let Some(row) = rows.next().unwrap() {
                        let swear_count: i32 = row.get(0).unwrap();
                        format!("@{} has sworn {} times in this chat", username, swear_count)
                    } else {
                        format!("@{} has not sworn in this chat. Good!", username)
                    }
                }
            }
        },
        Command::Stfu => {
            {
                let mut sil = silent.lock().unwrap();
                *sil = true;
            }
            "ok".to_string()
        },
        Command::Wakeup => {
            {
                let mut sil = silent.lock().unwrap();
                *sil = true;
            }
            "woken up".to_string()
        }
    };

    bot.send_message(msg.chat.id, text).await?;
 
    Ok(())
}

async fn handle_message(bot: Bot, msg: Message, db: Arc<Mutex<Connection>>, silent: Arc<Mutex<bool>>) -> ResponseResult<()> {
    eprintln!("Handling message: {:?}", msg.text());
    // get text from the message
    let text = match msg.text() {
        Some(text) => text,
        // this handles the case where the message is not text
        // for example when a user sends a photo
        // or when the bot is added
        None => return Ok(()),
    };

    // check for profanity
    let inappropriate: bool = text.is_inappropriate();

    if inappropriate {
        // send a message when inappropriate language is detected

        // get the chat
        let chat = msg.chat.clone();

        // check if the chat is a group
        if chat.is_group() || chat.is_supergroup() {
            // get the chat id
            let ChatId(chat_id) = chat.id;            
            // get the user id
            let UserId(user_id) = msg.from().unwrap().id;
            {
                // get the database connection
                let conn = db.lock().unwrap();
                // check if the user has sworn before
                let mut stmt = conn.prepare("SELECT swear_count FROM swearers WHERE user = ? AND chat_id = ?").unwrap();
                let mut rows = stmt.query(params![user_id.to_string(), chat_id]).unwrap();

                // if the user has sworn before
                if let Some(row) = rows.next().unwrap() {
                    // get the swear count
                    let swear_count: i32 = row.get(0).unwrap();
                    // increment the swear count
                    conn.execute("UPDATE swearers SET swear_count = ? WHERE user = ? AND chat_id = ?", params![swear_count + 1, user_id.to_string(), chat_id]).unwrap();
                } else {
                    // if the user has not sworn before
                    // insert the user into the database
                    conn.execute("INSERT INTO swearers (user, username, chat_id, swear_count) VALUES (?, ?, ?, ?)", params![user_id.to_string(), msg.from().unwrap().username.clone().unwrap_or(msg.from().unwrap().full_name()), chat_id, 1]).unwrap();
                }
            }
        }

        // get the naughty user
        let naughty_user = msg.from().unwrap().username.clone().unwrap_or(msg.from().unwrap().full_name());

        // make a message
        let message = messages::make_normal_swear_message(naughty_user);

        // get the current silent state
        let sil: bool;
        {
            sil = silent.lock().unwrap().clone();
        }

        if !sil {
            bot.send_message(msg.chat.id, message).await?;
        }
    }

    Ok(())
}

async fn handle_test_message(bot: Bot, msg: Message) -> ResponseResult<()> {
    // get text from the message
    let text = match msg.text() {
        Some(text) => text,
        // this handles the case where the message is not text
        // for example when a user sends a photo
        // or when the bot is added
        None => return Ok(()),
    };

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

    // load all stored singlish words from a saved text file "singlish.in"
    // and add them to the censor
    {
        use rustrict::{Trie, Type};
        use std::fs::File;
        use std::io::{self, BufRead};

        eprintln!("Adding singlish words to the censor...");
        // find the singlish file
        let singlish_file = File::open("./src/singlish.in").unwrap();
        let reader = io::BufReader::new(singlish_file);

        for line in reader.lines() {
            match line {
                Ok(line) => {
                    if line.is_empty() {
                        continue;
                    }
                    eprintln!("Adding word: {}", line);
                    unsafe {
                    Trie::customize_default().set(&line, Type::INAPPROPRIATE);
                    }
                }
                Err(e) => {
                    eprintln!("Error reading line: {}", e);
                }
            }
        }
    }

    run().await;
}

async fn run() {
    // initialize the bot
    pretty_env_logger::init();
    log::info!("Starting profanity bot...");

    let bot = Bot::from_env();
    let db: Arc<Mutex<Connection>> = Arc::new(Mutex::new(Connection::open(DATABASE).unwrap()));
    db.lock().unwrap().execute("
    CREATE TABLE IF NOT EXISTS swearers (
        id INTEGER PRIMARY KEY,
        user INTEGER NOT NULL,
        username TEXT NOT NULL,
        chat_id INTEGER NOT NULL,
        swear_count INTEGER NOT NULL
    )", ()).unwrap();
    // set up the dispatcher
    let mut silent_bool = false;
    let silent = Arc::new(Mutex::new(silent_bool));
    Dispatcher::builder(
        bot,
        Update::filter_message()
            .branch(
            // for commands
            dptree::entry()
                .filter_command::<Command>()
                .endpoint(handle_commands))
            .branch(
                // for case where context is in group
                dptree::filter(|msg: Message| msg.chat.is_group() || msg.chat.is_supergroup())
                    .endpoint(handle_message))
            .branch(
                // for everything else
                dptree::filter(|_msg: Message| true)
                    .endpoint(handle_test_message))
    )
    .dependencies(dptree::deps![db, silent])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}
