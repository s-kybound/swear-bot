use rand::{rngs::ThreadRng, Rng};

pub fn make_normal_swear_message(naughty_user: String) -> String {
    // use rng to select a random message
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..5) {
        0 => format!("@{} said a BAD WORD!", naughty_user),
        1 => format!("@{}, 10 cents to the swear jar!", naughty_user),
        2 => format!("Do you speak to your mother with that tone, @{}?", naughty_user),
        3 => format!("SWEARING, @{}? In this economy?", naughty_user),
        4 => format!("Mind your FUCKING LANGUAGE, @{}!", naughty_user),
        _ => panic!("rng generated another number i did not expect")
    }
}

pub fn make_blame_swear_message(blamer: String, naughty_user: String) -> String {
    let mut rng: ThreadRng = rand::thread_rng();
    match rng.gen_range(0..2) {
        0 => format!("@{} says that @{} said a BAD WORD!", blamer, naughty_user),
        1 => format!("@{}, thank you for your hard work in catching @{}", blamer, naughty_user),
        _ => panic!("rng generated another number i did not expect")
    }
}