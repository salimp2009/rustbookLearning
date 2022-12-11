use std::mem::{size_of, size_of_val};

fn main() {
    let s = "hello";
    println!("{s}");

    {
        let s2 = "world";
        println!("{s2}");
    } // lifetime of s2 end here
      // s2 here no more valid
      //    println!("{s2}");
    let s3 = String::from("demiros");
    let s4 = s3;
    // since s3 has Move trait only s3 not valid since assigned s4
    //    println!("s3 not valid yooow : {s3}");
    println!("s4: {s4}");
    println!("size of String: {}", size_of::<String>());
    println!("size of s4: {}", size_of_val(&s4));

    println!(
        "size of long String literal: {}",
        size_of_val("dsfsfsdfsdfsfd")
    );

    println!("length of s4 : {}", s4.len());

    let s5 = s4.clone();
    println!("s5: {s5}, s4: {s4}");

    let mut x = 42;
    let y = 43;

    let var1 = &x;
    println!("var1:{var1}");

    x = 45;

    let mut var2 = &x;

    //println!("var1 : {var1}");
    println!("var2: {var2}");

    var2 = &y;

    x = 100;
    println!("x: {x}, y: {y}, var2: {var2}");
}
