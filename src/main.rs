mod engine;
mod text_ui;
use std::env;

fn username() -> String {
    match env::var("USERNAME") {
        Ok(v) => return v,       
        Err(_) => return String::from("Unknown")
    };
}

fn main() {    
    println!("Hello, {}!",  username());

    let guess = engine::create_guess();

    guess.display_welcome();

    guess.display_lost_message();
    guess.display_win_message();
}

