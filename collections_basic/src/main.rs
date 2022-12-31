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
}
