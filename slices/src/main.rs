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

fn other_slices(s: &[i32]) {
    println!("received another slice: {:?}", s);
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
    let word1_with_slice = first_word_sliced(&s[..]);
    println!("sliced string first word : {word1} and {word1_with_slice}");

    let s2: &str = "hello, slice";
    let s2_word1 = first_word_sliced(s2);
    println!("s2 word1: {s2_word1}");

    let arr = [1, 2, 3, 4, 5];
    other_slices(&arr[1..3]);
    let slice: &[i32] = &arr[0..2];
    assert_eq!(slice, &[1, 2]);
}
