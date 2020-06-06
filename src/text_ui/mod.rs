impl super::engine::Guess {
    pub fn lost_message(& self) -> String{
        return format!("You've lost ! number was {}", self.number_to_guess);
    }

    pub fn display_lost_message(& self) {
        println!("{}", self.lost_message());
    }
}

#[cfg(test)]
mod tests {
    
    #[test]
    fn it_works() {
        // arrange & act
        let under_test = super::super::engine::create_guess();

        // assert
        assert!(under_test.lost_message().ends_with("42"))
    }
}