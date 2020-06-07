use rand::{thread_rng, Rng};

const MAXNUMBEROFATTEMPT: u16 = 10;

pub struct Guess {
   pub number_to_guess: u16,
   pub number_of_attempt : u16
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum GameStep {
    Win,
    Lost,
    TooSmall,
    TooLarge    
}

pub fn create_guess() -> Guess {
    Guess {
        number_to_guess: thread_rng().gen_range(0, 1000),
        number_of_attempt : 0
    }
}

impl Guess 
{
    pub fn what_next(& mut self, guess: u16) -> GameStep {
        self.number_of_attempt +=1 ;

        if guess == self.number_to_guess {return GameStep::Win; }

        if self.number_of_attempt == MAXNUMBEROFATTEMPT { return GameStep::Lost;}

        if guess < self.number_to_guess {return GameStep::TooSmall }

        return GameStep::TooLarge;    
    }
}


#[cfg(test)]
pub mod tests {
    pub fn create_guess(number_to_guess: u16, number_of_attempt: u16) -> super::Guess {
        super::Guess {
            number_to_guess: number_to_guess, number_of_attempt : number_of_attempt
        }
    }

    #[test]
    pub fn return_too_small(){

        //arrange
        let mut under_test = create_guess(42,0);

        //act
        let result = under_test.what_next(0);

        //assert
        assert_eq!(result, super::GameStep::TooSmall);
    }

    #[test]
    pub fn return_too_large(){

        //arrange
        let mut under_test = create_guess(42,0);

        //act
        let result = under_test.what_next(100);

        //assert
        assert_eq!(result, super::GameStep::TooLarge);
    }

    #[test]
    pub fn return_win(){

        //arrange
        let mut under_test = create_guess(42,0);

        //act
        let result = under_test.what_next(42);

        //assert
        assert_eq!(result, super::GameStep::Win);
    }

    #[test]
    pub fn return_lost(){

        //arrange
        let mut under_test = create_guess(42, super::MAXNUMBEROFATTEMPT - 1);

        //act
        let result = under_test.what_next(0);

        //assert
        assert_eq!(result, super::GameStep::Lost);
    }
}