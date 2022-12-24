use add_one;

fn main() {
    let num = 10;
    let new_num = add_one::add_one(num);
    println!("added one to {num} => new num: {new_num}");
}
