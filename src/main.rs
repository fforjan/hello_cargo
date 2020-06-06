mod engine;
mod text_ui;

fn main() {
    println!("Hello, {}!", " fred");

    let guess = engine::create_guess();

    guess.display_lost_message();
}

