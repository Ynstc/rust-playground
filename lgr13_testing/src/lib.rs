/* 13.0 testing assert */

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }
}

/* 13.1 testing assert_eq */

pub fn add_two(a: i32) -> i32 {
    a + 3
}

#[cfg(test)]
mod tests1 {
    use super::*;

    #[test]
    fn it_adds_two() {
        // assert_eq!(4, add_two(2));
        assert_ne!(4, add_two(2));
    }
}

/* 13.2 custom failure messages */

pub fn greeting(name: &str) -> String {
    format!("Hello {}", name)
    // format!("Hello ") //uncomment to check fail
}

#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }
}

/* 13.3 assert that function panic! */
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be higher than 1 or equal, got {}", value)
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}",
                value
            )
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests3 {
    use super::*;

    #[test]
    // #[should_panic]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
        // Guess::new(-200);
    }
}

/* 13.4 Test: returning Result type */

#[cfg(test)]
mod tests4 {

    #[test]
    fn it_works() -> Result<(), String> {
        // if 2 + 3 == 4 {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn it_works2() {
        assert_eq!(2 + 2, 4)
    }
}

// check option for"
// 1. `cargo test --help`  (cargo test command)
// 2. `cargo test -- --help` (resulting test binary)

/*  13.5 */

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests5 {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    // #[test]
    // fn this_test_will_fail() {
    //     let value = prints_and_returns_10(8);
    //     assert_eq!(5, value);
    // }
}

/* 13.6 running subset of tests */
pub fn add_two2(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests6 {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two2(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two2(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two2(100))
    }
}

// 1. can invoke one test by name:
// `cargo test one_hundred`
// or use plugin rust analyzer

// 2. run few test by typing part of test fn name:
// `cargo test add`

// 3. run test by test module name:
// `cargo test tests7::`

/* 13.7 ignoring tests */

mod test7 {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        //code that takes an hour to run
    }

    // if you want to run only ignored test:
    // `cargo test -- --ignored`
}

/* 13.8 integration tests */

pub fn add_two3(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests8 {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
