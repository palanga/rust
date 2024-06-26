fn main() {
    let mut s1 = String::from("hola");
    let len = calculate_length(&s1);
    println!("The length of <<{}>> is {}.", s1, len);

    change(&mut s1);
    println!("The length of <<{}>> is {}.", s1, calculate_length(&s1));
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str("lala");
}

fn dangle() -> &String {
    let s = String::from("holanda");
    &s
}