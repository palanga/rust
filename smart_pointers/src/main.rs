#![allow(dead_code)]
#![allow(unused_variables)]

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

fn main() {
    let b = Box::new(5);

    println!("{}", b);

    let list = List::Cons(2, Rc::new(List::Nil));

    println!("{:?}", list);


    let x = 5;
    let y = &x;
    let z = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);


    let xx = 5;
    let yy = Caja::new(xx);

    assert_eq!(5, xx);
    assert_eq!(5, *yy);

    let c = CustomSmartPointer {
        data: String::from("algo"),
    };

    let d = CustomSmartPointer {
        data: String::from("algo más"),
    };

    // c.drop(); // no compila
    drop(c);

    println!("acá!");

    use List::{Cons, Nil};

    let list_a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("{}", Rc::strong_count(&list_a));

    let list_b = Cons(3, Rc::clone(&list_a));
    println!("{}", Rc::strong_count(&list_a));

    {
        let list_c = Cons(4, Rc::clone(&list_a));
        println!("{}", Rc::strong_count(&list_a));
    }
    println!("{}", Rc::strong_count(&list_a));
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use super::*;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            // pedir prestado dos veces entra en pánico
            // let mut one_borrow = self.sent_messages.borrow_mut();
            // let mut two_borrow = self.sent_messages.borrow_mut();
            //
            // one_borrow.push(String::from(msg));
            // two_borrow.push(String::from(msg));
            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }

    #[test]
    fn a_single_item_tree() {
        let tree = Node::leaf(7);
        assert_eq!(tree.len(), 1);
        assert_eq!(tree.value, 7);
    }

    #[test]
    fn a_tree_with_a_branch_and_a_leaf() {
        let leaf = Node::leaf(7);
        let branch = Node::branch(11, leaf);
        assert_eq!(branch.len(), 2);
        assert_eq!(branch.value, 11);
        assert_eq!(branch.first_child().value, 7);
    }

    // ya me aburrí
    #[test]
    fn a_leaf_can_have_a_parent() {
        let leaf = Node::leaf(7);
        let branch = Node::branch(11, leaf);
        leaf.add_parent(branch);
        assert_eq!(branch.len(), 2);
        assert_eq!(branch.value, 11);
        assert_eq!(branch.first_child().value, 7);
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    pub fn first_child(&self) -> Rc<Node> {
        self.children.borrow_mut().first().unwrap().clone()
    }
}

impl Node {
    pub fn len(&self) -> usize {
        self.children.borrow().iter().fold(1, |acc, x| acc + x.len())
    }
}

impl Node {
    pub fn leaf(item: i32) -> Rc<Node> {
        Rc::new(Node {
            value: item,
            children: RefCell::new(vec![]),
        })
    }

    pub fn branch(item: i32, child: Rc<Node>) -> Rc<Node> {
        Rc::new(Node {
            value: item,
            children: RefCell::new(vec![child]),
        })
    }
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where
        T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("over");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("90%");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("75%")
        }
    }
}


struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

struct Caja<T>(T);

impl<T> Caja<T> {
    fn new(t: T) -> Caja<T> {
        Caja(t)
    }
}

impl<T> Deref for Caja<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
enum List<T> {
    // Cons(T, Box<List<T>>),
    Cons(T, Rc<List<T>>),
    Nil,
}

// impl<T> List<T> {
//     pub fn empty() -> List<T> {
//         List::Nil
//     }
// }
