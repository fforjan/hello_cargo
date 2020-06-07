extern crate readline;

impl super::engine::Guess {
    pub fn lost_message(& self) -> String{
        return format!("You've lost in {} attempts ! number was {}", self.number_of_attempt, self.number_to_guess);
    }

    pub fn win_message(& self) -> String{
        return format!("Win !it took you {} attempts to guess {}", self.number_of_attempt, self.number_to_guess);
    }

    pub fn display_lost_message(& self) {
        println!("{}", self.lost_message());
    }

    pub fn display_too_small(& self, wrong_guess: u16) {
        println!("{} was too small", wrong_guess);
    }

    pub fn display_too_large(& self, wrong_guess: u16) {
        println!("{} was too large", wrong_guess);
    }

    pub fn display_win_message(& self) {
        println!("{}", self.win_message());        
    }

    pub fn display_welcome(& self) {
        println!("Welcome to guess a number !");
    }

    pub fn read_guess(&self) -> u16 {
        println!("Please enter a number between 0 and 1000:");

        let mut result = 0;
        while result == 0 {
            let input = readline::readline("> ").unwrap();

            match  input.parse::<u16>() 
            {
                Ok(v) => result = v,
                Err(e) => println!("We cannot find an interger")
            }
        }
    
        return result;
    }

}

#[cfg(test)]
mod tests {
    
    #[test]
    fn it_works() {
        // arrange & act
        let under_test = super::super::engine::tests::create_guess(42, 0);

        // assert
        assert!(under_test.lost_message().ends_with("42"))
    }
}