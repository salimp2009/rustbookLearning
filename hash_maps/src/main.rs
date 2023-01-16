use std::collections::{hash_map::RandomState, HashMap};

#[allow(dead_code)]
fn hasmap_hashers() {
    let s = RandomState::new();
    let mut map1 = HashMap::with_hasher(s);
    map1.insert(1, 2);
}

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 15);
    scores.entry(String::from("Red")).or_insert(40);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("scores: {:?}", scores);
    let mut map2 = HashMap::new();

    "hello world wonderfull Rust world"
        .split_whitespace()
        .for_each(|c| {
            let count = map2.entry(c).or_insert(0);
            *count += 1;
        });

    println!("map2: {:?}", map2);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    assert_eq!(score, 10);
    let solar_distance = HashMap::from([
        ("Mercury", 0.4),
        ("Venus", 0.7),
        ("Earth", 1.0),
        ("Mars", 1.5),
    ]);
    println!("solar distance: {:?}", solar_distance);
    solar_distance
        .iter()
        .for_each(|(k, v)| println!("{k}: {v}"));
}
