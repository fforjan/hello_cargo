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

    let mut guess = engine::create_guess();

    guess.display_welcome();
        
    let mut done = false;

    while !done {
        match guess.what_next(45) {
            engine::GameStep::TooSmall => guess.display_too_small(45),
            engine::GameStep::TooLarge => guess.display_too_large(45),
            engine::GameStep::Win => { guess.display_win_message(); done = true; },
            engine::GameStep::Lost => { guess.display_lost_message(); done = true; },
        }
    }
}

