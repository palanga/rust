use gui::{Button, Screen, SelectBox};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("si"),
                    String::from("no"),
                    String::from("quizás"),
                ],
            }),
            Box::new(Button {
                width: 75,
                height: 10,
                label: String::from("ok"),
            }),
        ]
    };

    screen.run()
}


