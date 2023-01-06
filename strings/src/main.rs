use std::mem;

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
}
