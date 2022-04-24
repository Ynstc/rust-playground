
/* 7.0 mod basics */
//by default in child module's everything is private
//but from child module you have access to everything in parent module
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        // fn seat_at_table() {}
    }

    // mod serving {
    //     fn take_order() {}

    //     fn serve_order() {}

    //     fn take_payment() {}
    // }

    pub fn eat_at_restaurant() {
        //Absolute path
        crate::front_of_house::hosting::add_to_waitlist();

        //Relative path
        front_of_house::hosting::add_to_waitlist();
    }
}

/* 7.1  */
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        //super allowed us to reference the parent module (and specify function we want to call - serve_order
        super::serve_order();
    }

    fn cook_order() {}
}

/* 7.2 */
// struct and it fields are private by default
mod back_of_house {
    //At that point you can change toast but you cannot mutate Breakfast directly because seazonal_fruit is private
    pub struct Breakfast {
        pub toast: String,
        seazonal_fruit: String
    }

    impl Breakfast {
        pub fn summer (toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seazonal_fruit: String::from("peaches")
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
}

/* 7.3 enums in mod */
// enums are public by default
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

}

/* 7.4 bringing external module into scope */
use rand::{Rng, CryptoRng, ErrorKind::Transient};

// use std::io;
// use std::io::Write
// is the same like:
// use std::io{self, Write};

mod front_of_house;
use crate::front_of_house::hosting //bring hosting into scope //absolute path
// use self::front_of_house::hosting //bring hosting into scope //relative path
// pub use crate::front_of_house::hosting //bring hosting into scope //pub allowed to reexport it further

pub fn eat_at_restaurant() {
    let secret_number = rand::thread_rng().gen_range(1,101)
    // front_of_house::hosting::add_to_waitlist()
    // front_of_house::hosting::add_to_waitlist()
    // front_of_house::hosting::add_to_waitlist()
    // front_of_house::hosting::add_to_waitlist() could be written as

    hosting::add_to_waitlist()
    hosting::add_to_waitlist()
    hosting::add_to_waitlist()
    hosting::add_to_waitlist()

    // functions: a good rule is to bring parent module in to scope so you can easily see that this function is external
    // structs, enums, other items: then full path
}
