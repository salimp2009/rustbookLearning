use std::ops::Rem;

fn with_if() {
    let number = 3;

    if number < 5 {
        println!("got the right number; lucky!");
        // else is optional
        // } else {
        //     println!("sorry maybe next time");
    }
}

#[derive(PartialEq, Debug)]
struct SplitSlice<'a, T: 'a> {
    slice: &'a [T],
}

impl<'a, T> Rem<usize> for SplitSlice<'a, T> {
    type Output = Self;

    fn rem(self, modulus: usize) -> Self::Output {
        let len = self.slice.len();
        let rem = len % modulus;
        let start = len - rem;
        Self {
            slice: &self.slice[start..],
        }
    }
}

// If we were to divide &[0, 1, 2, 3, 4, 5, 6, 7] into slices of size 3,
// the remainder would be &[6, 7].

fn with_if_elseif<T: Rem<usize> + Copy>(x: T)
where
    <T as Rem<usize>>::Output: PartialEq<usize>,
{
    if x % 4 == 0 {
        println!("number is divisible by 4");
    } else if x % 3 == 0 {
        println!("number is divisible by 3");
    } else if x % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 2, 3 or 4");
    }
}

#[allow(unused)]
fn with_different_return() {
    let condition = true;

    let _number = if let true = condition { 5 } else { 6 };
    //
    //    This wont work since branches dont have same return type
    //    let _number2 = if let true = condition { 5 } else { "something else" };
}

fn values_from_loop() -> i32 {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    result
}

fn loops_labels() -> i32 {
    let mut count = 0;

    'count_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'count_up;
            }

            remaining -= 1;
        }
        count += 1;
    }
    count
}

fn loops_with_while() -> i32 {
    let mut number = 3;

    while number != 0 {
        println!("while loop number : {number}");
        number -= 1;
    }
    number
}

fn loops_with_for() {
    let array1 = [10, 20, 30, 40, 50];

    for element in array1 {
        println!("element: {element}");
    }
}

fn loops_with_ranges() {
    for num in (1..5).enumerate().rev() {
        println!("index: {:?}, value : {:?}", num.0, num.1);
    }
    for elem in [1, 2, 3, 4, 5].iter().enumerate() {
        println!("elem: {:?}, elem : {:?}", elem.0, elem.1);
    }
}

fn main() {
    with_if();
    with_if_elseif(5);

    assert_eq!(
        SplitSlice {
            slice: &[0, 1, 2, 3, 4, 5, 6, 7]
        } % 3,
        SplitSlice { slice: &[6, 7] }
    );

    let split01 = SplitSlice {
        slice: &[0, 1, 2, 3, 4, 5, 6, 7],
    };

    let result = split01 % 3;

    let sresult = format!("result: {:?}", result.slice);
    println!("splitted slice : {sresult}");
    println!("result from loop {}", values_from_loop());
    println!("result from loops_labels {}", loops_labels());
    println!("loops_with_while: number {}", loops_with_while());
    loops_with_for();
    loops_with_ranges();
}
