use std::{fs::File, io::ErrorKind};

fn recover_error() {
    let _greeting_file_result = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("./hello.txt").unwrap_or_else(|error| {
                panic!("problem creating file: {:?}", error);
            })
        } else {
            panic!("problem creating file: {:?}", error);
        }
    });
}

fn expect_error() -> File {
    let greeting_file_result =
        File::open("hello.txt").expect("please include 'hello.txt' in this project");
    greeting_file_result
}

fn main() {
    // panic!("crash and run");
    let _v = vec![1, 2, 3];
    // _v[99];

    recover_error();
    let _file = expect_error();
}
