fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{}", s1);
    println!("{}", s2);

    let s = String::from("hola");
    let x = 5;

    takes_ownership(s.clone());
    makes_copy(x);
    takes_ownership(s);
    makes_copy(x);


    let ss1 = gives_ownership();
    let ss2 = String::from("holanda");
    let ss3 = takes_and_gives_back(ss2);

    let (sss2, len) = calculate_length(ss3);

    println!("{}", sss2);

}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
