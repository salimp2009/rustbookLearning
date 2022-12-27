use crate::front_of_house::hosting::seat_at_table;
use front_of_house::hosting::add_to_waitlist;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[allow(dead_code)]
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        pub fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {
            super::hosting::seat_at_table();
        }

        fn serve_order() {
            take_order();
        }

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    add_to_waitlist();
    seat_at_table();

    let mut meal = back_of_house::Breakfast::summer("rye");
    meal.toast = String::from("whole wheat");
    println!("would like to have a {} toast please", meal.toast);
}

pub fn eat_at_restaurant2() {
    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;
}

fn deliver_order() {}

#[allow(dead_code)]
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    pub enum Appetizer {
        Soup,
        Salad,
    }

    // struct  fields are private by default
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Self {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("strawberry"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
