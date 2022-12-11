use std::fmt::Display;

fn another_function() {
    println!("another function so what!");

    fn inside_funct() {
        println!("I am function defined inside a function");
    }

    inside_funct();
}

fn another_value<T: Display>(x: T) {
    println!("another value: {x}");
}

// #[allow(unused)]
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn statement_expression() {
    let x = {
        let y = 6;
        y + 1
    };
    println!("x from expression: {:?}", x);
}

fn main() {
    another_function();
    another_value(5);
    statement_expression();
    let x = five();
    println!("returning from a function x: {x}");
    //    assert_eq!(1, 1);
    println!("plus_one; {}", plus_one(5));
}
