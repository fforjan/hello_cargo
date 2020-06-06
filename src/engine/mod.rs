pub struct Guess {
   pub number_to_guess: u16
}

pub fn create_guess() -> Guess {
    Guess {
        number_to_guess:42
    }
}
