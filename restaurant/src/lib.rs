// idomatic Rust wat of specifiying the parent module
// using pub reexport front_of_house module and make all pub
// functions available: see chapter 14 for a more clear explanation
pub use crate::front_of_house::hosting;

// idiomatic way of Rsut specifiying full path enums, structs
use std::collections::HashMap;
mod front_of_house;

// example of glob operator that brings all public items
// this is mostly used for testing otherwise it will bring unneccessary items
#[allow(unused_imports)]
use std::collections::*;

// example of using both io and io::Write in module import
// self refers std::io
#[allow(unused_imports)]
use std::io::{self, Write};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[allow(dead_code)]
mod customer {
    pub fn eat_at_restaurant() {
        super::hosting::add_to_waitlist();
        super::hosting::seat_at_table();

        let mut meal = super::back_of_house::Breakfast::summer("rye");
        meal.toast = String::from("whole wheat");
        println!("would like to have a {} toast please", meal.toast);
    }
}

pub fn eat_at_restaurant2() {
    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;
    hosting::add_to_waitlist();
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

pub fn sample_hashmap() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    println!("{}", map[&1]);
}

pub fn book_reviews_hashmap() {
    let mut book_reviews = HashMap::new();
    book_reviews.insert("Adventures of Demir".to_string(), "MasterPiece".to_string());
    book_reviews.insert(
        "Grimms' Fairy Tales".to_string(),
        "Masterpiece.".to_string(),
    );
    book_reviews.insert(
        "Pride and Prejudice".to_string(),
        "Very enjoyable.".to_string(),
    );
    book_reviews.insert(
        "The Adventures of Sherlock Holmes".to_string(),
        "Eye lyked it alot.".to_string(),
    );

    let books_to_find = ["Adventures of Demir", "Grimms' Fairy Tales"];

    for &book in &books_to_find {
        match book_reviews.get(book) {
            Some(review) => println!("{book}: {review}"),
            None => println!("{book} is unreviewed!"),
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
