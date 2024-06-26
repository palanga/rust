use std::rc::Rc;
use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let lala = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("hola");
        thread::sleep(Duration::from_millis(100));
        println!("y chau");
        println!("{:?}", lala);
    });

    // println!("{:?}", lala); // no compila

    println!("Hello, world!");
    handle.join().unwrap();
    // thread::sleep(Duration::from_millis(200));

    // ######################################
    // ### channels
    // ######################################

    let (sender, receiver) = mpsc::channel();
    let sender_b = sender.clone();

    thread::spawn(move || {
        let a = String::from("hola de nuevo");
        let b = String::from("que tal");
        sender.send(a).unwrap();
        thread::sleep(Duration::from_millis(600));
        // println!("{}", a); // no compila
        sender.send(b).unwrap();
    });

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        let a = String::from("todo bien");
        let b = String::from("co tas?");
        sender_b.send(a).unwrap();
        sender_b.send(b).unwrap();
    });


    receiver.iter().for_each(|item| println!("{}", item));


    // ######################################
    // ### mutex
    // ######################################

    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("{:?}", m);


    // #######

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}", *counter.lock().unwrap());

}
