use std::mem;

#[allow(dead_code)]
fn indexing_strings() {
    let _s = String::from("hello astro");
    //    direct access via index is not allowed Index<Integer> not implemented for String
    //    let h = s[0];
}

#[allow(dead_code)]
fn strings_internal_rep() {
    let hello = String::from("Hola");
    assert_eq!(hello.len(), 4);

    let hello = String::from("Здравствуйте");
    let size = hello.as_bytes().iter().map(mem::size_of_val).sum::<usize>();
    println!("size of String: {hello} is {size}");
    assert_eq!(hello.len(), size);
    let size = mem::size_of_val(&hello);
    println!("size_of_val result of {hello} is {size}");
}

fn main() {
    let data = "initial contents";

    // s is UTF8 ; every char is 4 bytes
    let s: &str = "💖💖💖💖💖";
    assert_eq!(s.len(), 20);
    println!("hearts: {s}");

    //s is ASSCII; so every char is one byte
    let s: &str = "hello";
    assert_eq!(s.len(), 5);

    let s: [char; 5] = ['h', 'e', 'l', 'l', 'o'];
    assert_eq!(s.len(), 5);
    let size = s.iter().map(mem::size_of_val).sum::<usize>();
    assert_eq!(size, 20);

    let _s1 = data.to_string();

    let _s2 = "more contents".to_string();

    let hello = String::from("안녕하세요");
    println!("hello: {hello}");
    assert_eq!(hello.len(), 15);

    let hello = String::from("hello");
    assert_eq!(hello.len(), 5);

    let mut s = String::from("foo");
    s.push_str("bar");
    assert_eq!(s, "foobar");

    let s1 = "this is old";
    assert_eq!("this is new", s1.replace("old", "new"));

    let s2 = "zoo is awesome";
    s.push_str(s2);
    println!("s2 is still valid: {s2}");
    println!("s is changed: {s}");

    let s = String::from("hello");
    let s1 = String::from(" lightning");
    let s = s + &s1;
    println!("s: {s}");
    println!("s1: {s1}");

    let s = String::from("tic");
    let s1 = String::from("tac");
    let s3 = String::from("toe");
    let s2 = format!("{s}-{s1}-{s3}");
    println!("{s2}");
    assert_eq!("tic", s);

    strings_internal_rep();

    let hello = "Здравствуйте";
    let slice1 = &hello[0..6];
    println!("slice of hello: {slice1}");

    "Зд".chars().for_each(|c| println!("{c}"));
    "Зд".bytes().for_each(|c| println!("{c}"));
}
