fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_sliced(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    s
}

fn main() {
    let mut s = String::from("demiros");
    let length_or_last = first_word(&s);
    s.clear();
    println!("length or last index: {length_or_last}");
    s.push_str("Hello World");
    let hello = &s[..5];
    let world = &s[5..];
    let hello_world = &s[..];
    println!("{hello}{world}");
    println!("{hello_world}");
    s.insert_str(0, "hoops ");

    let word1 = first_word_sliced(&s);
    println!("sliced string first word : {:?}", word1);
}
