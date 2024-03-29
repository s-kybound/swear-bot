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
        0 => format!("@{} is innocent, @{}!", naughty_user, blamer),
        1 => format!("@{} is not the one who said the bad word, @{}!", naughty_user, blamer),
        2 => format!("@{} is a good person, @{}!", naughty_user, blamer),
        3 => format!("@{} is a saint, @{}!", naughty_user, blamer),
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
    let mut rng2: ThreadRng = rand::thread_rng();

    // a list of items we can compare the cost to
    let (item, item_cost) = match rng.gen_range(0..14) {
        0 => ("dh coffee".to_string(), 6.00),
        1 => ("fc bandung".to_string(), 1.60),
        2 => ("mcchicken alacarte".to_string(), 2.50),
        3 => ("waacow beef bowl".to_string(), 12.90),
        4 => ("nus school fee (default)".to_string(), 8000.00),
        5 => ("nus school fee (scholar)".to_string(), 4000.00),
        6 => ("djungelskog".to_string(), 39.90),
        7 => ("nasi lemak".to_string(), 3.50),
        8 => ("casa2 gopizza savers deal".to_string(), 2.80),
        9 => ("8gb ram 256gb m2 macbook air".to_string(), 1599.00),
        10 => ("rc4 summer stay".to_string(), 1800.00),
        11 => ("bus from nus to ntu".to_string(), 2.50),
        12 => ("hwangs galbitang".to_string(), 7.00),
        13 => ("singular dodgeball (according to karthik)".to_string(), 20.00),
        _ => panic!("rng generated another number i did not expect")
    };

    match rng2.gen_range(0..4) {
        0 => format!("did you know that ${:.2} is equivalent to {:.5} of {}?", cost, cost / item_cost, item),
        1 => format!("${:.2} is the cost of {:.5} of {}!", cost, cost / item_cost, item),
        2 => format!("you could buy {:.5} of {} with ${:.2}!", cost / item_cost, item, cost),
        3 => format!("${:.2} is the cost of {:.5} of {}! wow!", cost, cost / item_cost, item),
        _ => panic!("rng generated another number i did not expect")
    }
}