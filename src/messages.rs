use rand::{rngs::ThreadRng, Rng};

pub fn make_normal_swear_message(naughty_user: String) -> String {
    // use rng to select a random message
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..5) {
        0 => format!("@{} said a BAD WORD!", naughty_user),
        1 => format!("@{}, 10 cents to the swear jar!", naughty_user),
        2 => format!("do you speak to your mother with that tongue, @{}?", naughty_user),
        3 => format!("SWEARING, @{}? In this economy?", naughty_user),
        4 => format!("mind your LANGUAGE, @{}!", naughty_user),
        5 => format!("NAUGHTY NAUGHTY, @{}!", naughty_user),
        _ => panic!("rng generated another number i did not expect")
    }
}

pub fn blamer_is_innocent_message(blamer: String, naughty_user: String) -> String {
    // use rng to select a random message
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..3) {
        0 => format!("@{} is innocent, @{}!", blamer, naughty_user),
        1 => format!("@{} is not the one who said the bad word, @{}!", blamer, naughty_user),
        2 => format!("@{} is a good person, @{}!", blamer, naughty_user),
        3 => format!("@{} is a saint, @{}!", blamer, naughty_user),
        _ => panic!("rng generated another number i did not expect")
    }
}

pub fn make_blame_swear_message(blamer: String, naughty_user: String) -> String {
    let mut rng: ThreadRng = rand::thread_rng();
    match rng.gen_range(0..3) {
        0 => format!("@{} says that @{} said a BAD WORD!", blamer, naughty_user),
        1 => format!("@{}, thank you for your hard work in catching @{}!", blamer, naughty_user),
        2 => format!("\"@{}, NAUGHTY NAUGHTY!\"\n-@{}", naughty_user, blamer),
        3 => format!("@{} 1, @{} 0!", blamer, naughty_user),
        _ => panic!("rng generated another number i did not expect")
    }
}

pub fn cost_fun_fact(cost: f64) -> String {
    let mut rng: ThreadRng = rand::thread_rng();

    // a list of items we can compare the cost to
    let (item, item_cost) = match rng.gen_range(0..4) {
        0 => ("dh coffee".to_string(), 6.00),
        1 => ("fc bandung".to_string(), 1.60),
        2 => ("mcchicken alacarte".to_string(), 2.50),
        3 => ("waacow beef bowl".to_string(), 12.90),
        _ => panic!("rng generated another number i did not expect")
    };

    match rng.gen_range(0..4) {
        0 => format!("did you know that ${:.2} is equivalent to {} of {}?", cost, cost / item_cost, item),
        1 => format!("${:.2} is the cost of {} of {}!", cost, cost / item_cost, item),
        2 => format!("you could buy {} of {} with ${:.2}!", cost / item_cost, item, cost),
        3 => format!("${:.2} is the cost of {} of {}! wow!", cost, cost / item_cost, item),
        _ => panic!("rng generated another number i did not expect")
    }
}