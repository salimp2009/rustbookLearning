fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);

    let a = [1, 2, 3];
    let v = a.iter().map(|x| x * 2).collect::<Vec<i32>>();
    println!("v : {:?}", v);

    let v = vec![1, 2, 3, 5];
    println!("v : {:?}", v);

    let third: &i32 = &v[2];

    println!("third: {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(x) => {
            println!("third : {x}");
        }
        None => {
            println!("out of bounds!");
        }
    }

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    println!("first: {first}");
    v.push(6);

    //    this wont compile since we are using mut reference access in between
    //    println!("first : {first}");
    println!("first : {}", v[0]);

    for i in &v {
        println!("{i}")
    }
    for i in &mut v {
        *i += 10;
    }

    println!("{:?}", v);
    v.iter_mut().for_each(|i| {
        *i += 10;
    });

    println!("{:?}", v);
}
