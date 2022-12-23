#[derive(Debug)]
#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cent(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => {
            println!("luck the luke Dime");
            10
        }
        Coin::Quarter => 25,
    }
}

#[allow(dead_code)]
enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsStates),
}

#[derive(Debug)]
#[allow(dead_code)]
enum UsStates {
    Alabama,
    Alaska,
    California,
    NewYork,
    // ....bla bla
}

fn value_in_cent2(coin: Coin2) -> u8 {
    match coin {
        Coin2::Penny => 1,
        Coin2::Nickel => 5,
        Coin2::Dime => {
            println!("luck the luke Dime");
            10
        }
        Coin2::Quarter(state) => {
            println!("State quarter from: {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    //     match x {
    //         None => None,
    //         Some(val) => Some(val + 1),

    // }
    // Alternative to above recommended by clippy
    x.map(|val| val + 1)
}

fn dice_roll() {
    let dice = 9;
    match dice {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        //        other => move_player(other),
        _ => (),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}

#[allow(dead_code)]
fn move_player(_num_spaces: u8) {}

fn main() {
    let penny_in_cents = value_in_cent(Coin::Penny);
    println!("penny in cents: {penny_in_cents}");
    value_in_cent(Coin::Dime);
    value_in_cent2(Coin2::Quarter(UsStates::California));
    let some_value = plus_one(Some(5));
    assert_eq!(some_value, Some(6));
    dice_roll();
}
