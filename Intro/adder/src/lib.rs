pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // fn can_hold(&self, other: &Rectangle) -> bool {
    //     self.width > other.width && self.height > other.height
    // }

 fn can_hold(&self, other: &Rectangle) -> bool {
        self.width < other.width && self.height > other.height
    }
}


pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}





pub struct Guess {
    value: i32,
}
pub struct GuessB {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }
}


impl GuessB {
    pub fn new(value: i32) -> GuessB {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {value}."
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {value}."
            );
        }

        GuessB { value }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }


    #[test]
    fn exploration2() {
        let result = add(2, 3);
        assert_eq!(result, 9);
    }



    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
}


 #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1
        };

        assert!(!smaller.can_hold(&larger));
    }




      #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{result}`"
        );
    }




    // Should panic if the value is less than 100


     #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }


        #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100b() {
        GuessB::new(200);
    }
}







