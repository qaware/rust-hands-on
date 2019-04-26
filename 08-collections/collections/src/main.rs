fn main() {
    vector();

    strings();

    hashmap();
}

fn hashmap() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("s: {:?}", scores);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores); // Blue -> 10, Yellow -> 50
}

fn strings() {
    let mut s = String::new();
    println!("s: {}", s);

    let s2 = "initial contents".to_string();
    println!("s2: {}", s2);

    let s3 = String::from("initial contents");
    println!("s3: {}", s3);

    s.push_str("bar");
    println!("s: {}", s);

    let s1: String = String::from("Hello, ");
    let s2: String = String::from("world!");
    let s3: String = s1 + &s2;

    // Doesn't compile
    // println!("s1: {}", s1);

    println!("s3: {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    println!("s: {}", s);

    let s1 = String::from("hello");
    // Doesn't compile
    // let h = s1[0];

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    let s = "test";
    let string = s.to_string();
    let s2 = string.as_str();
}

fn vector() {
    let v: Vec<i32> = Vec::new();
    println!("v: {:?}", v);

    let v2 = vec![1, 2, 3];
    println!("v2: {:?}", v2);

    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);
    println!("v3: {:?}", v3);

    let third: &i32 = &v2[2];
    println!("The third element is {}", third);

    match v2.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let first = &v3[0];
    // Doesn't compile
    // v3.push(6);
    println!("The first element is: {}", first);

    for i in &v2 {
        println!("{}", i);
    }
}
