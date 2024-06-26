fn main() {
    let hola_que_tal = String::from("hola, que tal");

    let word = first_word(&hola_que_tal);

    // hola_que_tal.clear();

    println!("{}", word);

    // let hola = &hola_que_tal[0..5];
    // let que = &hola_que_tal[6..9];

    // println!("{}", hola);
    // println!("{}", que);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s
}


