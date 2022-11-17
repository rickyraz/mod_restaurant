mod front_of_houses;

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        // seasonal fruit still private <-----------
        seasonal_fruit: String,
    }
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

// use crate::front_of_houses::hosting::add_to_waitlist;

use front_of_houses::hosting;
use front_of_houses::hosting::add_to_waitlist;

mod customer {
    pub fn eat_at_restaurant() {
        // needs super keyword for reach the global crate
        super::hosting::add_to_waitlist();
    }
}

pub fn eat_at_restaurant() {
    add_to_waitlist();
    hosting::add_to_waitlist();

    //---------

    // -- Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();

    // -- Relative path
    // front_of_house::hosting::add_to_waitlist();

    // -- simplify the path
    hosting::add_to_waitlist();

    //---------

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");

    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
