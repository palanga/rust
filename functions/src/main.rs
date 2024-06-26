fn main() {
    let a = hola(4);

    println!("{}", a);


    let number = 10;

    if number == 0 {
        println!("condition true")
    }

    let a = [4, 6, 2, 4, 7];
    let mut index = 0;

    while index < a.len() {
        println!("the value is {}", a[index]);
        index += 1;
    }

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{}", number)
    }

}

fn hola(n: i32) -> i64 {
    (n + 1).into()
}
