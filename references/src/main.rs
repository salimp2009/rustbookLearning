fn two_mutable_references() {
    let mut s = String::from("my friend demir");

    let r1 = &mut s;
    println!("{r1}");
    let r2 = &mut s;
    println!("{r2}");
    //    this will not compile you cant use more than
    //    mutable reference
    //    println!("{r1}, {r2}");
}

fn mutable_immutable_refs() {
    let mut s = String::from("my sisteros Didemos");
    let r1 = &s;
    let r2 = &s;
    // cannot have mut ref since there are immutable references
    //    let r3 = &mut s;
    println!("{r1} {r2}");
    let r3 = &mut s;
    println!("super --->> {r3}");
}

// fn dangle() -> &String {
//     &String::from("no danglinng allowed!")
// }

fn dangle() -> String {
    String::from("no danglinng allowed!")
}

fn main() {
    two_mutable_references();
    mutable_immutable_refs();
    let ref_to_nothing = dangle();
    println!("{ref_to_nothing}");
}
