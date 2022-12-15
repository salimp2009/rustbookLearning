fn change_mut_string(s: &mut String) {
    s.insert_str(0, "ok ok changed");
    println!("changed mut String: {s}");
}

fn gives_ownership() -> String {
    String::from("king of the hill")
}

fn calculate_length_byref(s: &String) -> usize {
    s.len()
}

fn takes_givesback_ownership(mut s: String) -> String {
    s.push_str(" giving back");
    s + "is this "
}
fn strlen_dyn2(s: &dyn AsRef<str>) -> usize {
    s.as_ref().len()
}

fn calculate_length(s: String) -> (String, usize) {
    let len: usize = s.len();
    (s, len)
}

fn takes_ownership(some_string: String) {
    println!("takes_ownership: {some_string}");
}

fn makes_copies(some_value: i32) {
    println!("made copy of a value: {some_value}");
}

fn main() {
    let s = String::from("hello demiros");

    takes_ownership(s);
    //    println!("s is moved into function : {s}");
    let x = 5;
    makes_copies(x);
    println!("x is still valid here : {x}");

    let y: &dyn AsRef<str> = &"demiros";
    println!("length of y(Trait object): {}", strlen_dyn2(y));

    // Trait object example, function strlen_dyn2 accepts any typele
    // that implement AsRef trait; but the only thing that can be done as_ref() function
    // or if it is another Trait you can only call that Traits methods as the type implements
    // Trait object does type erasure
    let x: Box<dyn AsRef<str>> = Box::new(String::from("didokyos"));
    println!("length of x(Trait Object): {}", strlen_dyn2(x.as_ref()));

    let s1 = gives_ownership();
    println!("new owner s1: {s1}");

    let s2 = String::from("yoooe new hoomy:)");
    let s3 = takes_givesback_ownership(s2);
    println!("take-give ownership s3: {s3}");

    let (s4, len) = calculate_length(s3);
    println!("s4: {s4}, length : {len} ");

    let str_length = calculate_length_byref(&s4);
    println!("length of \"{s4}\": {str_length}");
    println!("s4 is still valid: {s4}");

    let mut s5 = String::from("change me change me");
    change_mut_string(&mut s5);
}
