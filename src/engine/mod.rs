use rand::{thread_rng, Rng};

pub struct Guess {
   pub number_to_guess: u16,
   pub number_of_attempt : u16
}

pub fn create_guess() -> Guess {
    Guess {
        number_to_guess: thread_rng().gen_range(0, 1000),
        number_of_attempt : 0
    }
}

#[cfg(test)]
pub mod tests {
    pub fn create_guess(number_to_guess: u16, number_of_attempt: u16) -> super::Guess {
        super::Guess {
            number_to_guess: number_to_guess, number_of_attempt : number_of_attempt
        }
    }
}