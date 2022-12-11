use std::io;

const THREE_HOURS_IN_SECONDS: u32 = 30 * 20 * 15;

fn foo<const N: usize>(arr: &[i32; N]) {
    let _x: [i32; N];
    print!("arr: ");
    for i in arr {
        print!(" {i} ");
    }
    println!("\nwith N: {}", N * 2);
}

fn mytypes() {
    let mydecimal = 53u8;
    println!("mydecimal(53u8): {mydecimal}");
    let mybyte = b'A';
    println!("mybyte(b'A'): {mybyte}");
    let myhex = 0xf0;
    println!("myhex(0xf0): {myhex}");
    //   this overflow and does not compile ..nice ..even in release mode
    //   let myu8: u8 = 256;
    //   println!("my owerfleeew u8: {myu8}");
    let _myfloat1 = 2.0; // default f64
    let _myfloat2: f32 = 24.0;
}

fn myshadow() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the scope = {x}");
    }

    println!("The value of x in the scope = {x}");
}

fn shadow2() {
    let spaces = "      ";
    let spaces = spaces.len();
    println!("non mutable shadows: {spaces} ");
}

// fn shadow3() {
//     let mut spaces = "      ";
//     spaces = spaces.len();
//     println!("mutable shadows: {spaces} ");
// }
fn numericops() {
    let remainder = 43 % 5;
    println!("remainder : {remainder}");
}

fn mychars() {
    let _z = 'c';
    let _zz: char = 'C';
    let mycat = '😻';
    println!("my cat : {mycat}");
}

fn mystrings() {
    let s1 = String::from("Rock");
    let s2 = String::from(" Scissor");
    let s3 = s1 + &s2;
    // String does not implement Copy trait so it has been moved
    //    println!("s1 (is moved!! : {s1}");
    println!("s3 : {s3}, s2(is browwed): {s2}");
}

fn mytuples() {
    let mytup1: (i32, f64, u8) = (500, 6.4, 123);
    let (x, y, z) = &mytup1;
    println!("x: {x}, y: {y}, z:{z}");
}

fn myarrays() {
    let myarr1: [i32; 5] = [1, 2, 3, 4, 5];
    println!("my array: {}", myarr1[0]);
    let sum = myarr1.iter().sum::<i32>();
    println!("sum : {sum}");
    for i in myarr1 {
        println!("{}", i);
    }

    let arr2: [i32; 3] = [1, 2, 3];

    let s = format!("arr2: {:?}", arr2);
    println!("{s}");

    // [value to be assigned ; the number of elements]
    // so value = 3 , count = 4 in this example
    let arr3: [i32; 4] = [3; 4];
    let s = format!("arr3: {:?}", arr3);
    println!("{s}");

    // this is out of bound error at runtime but it compiles !!
    // println!("try accessing arr3 5th elem: {}", arr3[7]);
    // let val2 = arr3[5];
    // println!("{val2}");
}

#[allow(dead_code)]
fn arraycheck() {
    let arr = [1, 2, 3, 5];

    let mut index = String::new();

    println!("Please enter an Index:");

    io::stdin().read_line(&mut index).expect("Failed to read");

    let index = index.trim().parse::<usize>().expect("invalid index");

    let element = arr[index];

    println!("element: {element}");
}

fn slicingarrays() {
    let mut array: [i32; 3] = [0; 3];

    array[1] = 1;
    array[2] = 2;
    assert_eq!([1, 2], &array[1..]);
    for i in array {
        println!("slicing arrays: {i}");
    }
}

fn slicetoarray() {
    let bytes: [u8; 3] = [1, 0, 2];

    let val1: u8 = 0;
    assert_eq!(
        val1,
        u8::from_le_bytes(<[u8; 1]>::try_from(&bytes[1..2]).unwrap())
    );

    assert_eq!(512, u16::from_le_bytes(bytes[1..3].try_into().unwrap()));

    let val2 = u16::from_le_bytes(bytes[0..2].try_into().unwrap());
    println!("val2: {val2}");
}

fn main() {
    let mut x = 5;
    println!("the value of x {x}");
    x = 6;
    println!("the new value of x {x}");
    println!("the value of THREE_HOURS_IN_SECONDS = {THREE_HOURS_IN_SECONDS:}");
    let myvec: [i32; 3] = [1, 2, 3];
    foo(&myvec);

    foo::<5>(&[1, 2, 3, 4, 5]);

    let y = 5;
    let y = y + 5;
    println!("value of y: {y}");
    myshadow();
    shadow2();
    //shadow3();
    mytypes();
    numericops();
    mychars();
    mystrings();
    mytuples();
    myarrays();
    //    arraycheck();
    slicingarrays();
    slicetoarray();
}
