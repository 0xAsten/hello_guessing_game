// vec
// String
// hashmap
use std::collections::HashMap;

fn vector() {
    let mut v: Vec<i32> = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);
    println!("{:?}", v);

    let third = &v[2];
    println!("{}", third);

    match v.get(4) {
        Some(i) => println!("{}", i),
        None => println!("There is not third element"),
    }

    if let Some(i) = v.get(2) {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{}", i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from(" blue")),
    ];

    match &row[1] {
        SpreadsheetCell::Float(i) => println!("{}", i),
        _ => println!("not a float"),
    }
}

fn string() {
    let mut s = String::from("foo");
    s.push_str(" bar");
    s.push('!');
    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = String::from("world");
    let s3 = s1 + &s2;
    println!("{}", s3);

    let s1 = String::from("你好");
    let s4 = format!("{} {}", s1, s2);
    println!("{}", s4);

    for b in s4.chars() {
        println!("{}", b);
    }
}

fn hashmap() {
    let mut scores = HashMap::new();
    scores.insert("blue", 10);
    scores.insert("yellow", 20);

    println!("{:#?}", scores);

    let score = scores.get("blue");
    if let Some(i) = score {
        println!("{}", i);
    }

    scores.insert("blue", 30);
    let score = scores.get("blue");
    if let Some(i) = score {
        println!("{}", i);
    }

    let value = scores.entry("blue").or_insert(50);
    *value += 50;
    let score = scores.get("blue");
    if let Some(i) = score {
        println!("{}", i);
    }

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", map);
}

fn main() {
    hashmap();
}
