use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    println!("{:?}", v);

    let mut vv = vec![1, 2, 3];

    v.push(5);

    println!("{:?}", v);
    println!("{:?}", vv);

    v.pop();

    println!("{:?}", v);
    v.pop();
    println!("{:?}", v);

    // &vv[6];


    for i in &mut vv {
        *i += 50;
    }
    println!("{:?}", vv);

    for c in "Зд".chars() {
        println!("{c}");
    }

    for c in "Зд".bytes() {
        println!("{c}");
    }



    let mut scores = HashMap::new();

    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);

    println!("{:#?}", scores);
    println!("{:#?}", scores.get("Blue").copied().unwrap_or(0));

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

}
