impl super::engine::Guess {
    pub fn lost_message(& self) -> String{
        return format!("You've lost ! number was {}", self.number_to_guess);
    }

    pub fn win_message(& self) -> String{
        return format!("Win !it took you {} attempts to guess {}", self.number_of_attempt, self.number_to_guess);
    }

    pub fn display_lost_message(& self) {
        println!("{}", self.lost_message());
    }

    pub fn display_win_message(& self) {
        println!("{}", self.win_message());        
    }

    pub fn display_welcome(& self) {
        println!("Welcome to guess a number !");
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