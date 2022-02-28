fn main() {
    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1, 2),
        Action::ChangeColorRGB(1, 2, 3),
    ];
    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{}", s);
            }
            Action::MoveTo(x, y) => {
                println!("{}, {}", x, y);
            }
            Action::ChangeColorRGB(r, g, b) => {
                println!("{}, {}, {}", r, g, b);
            }
        }
    }
}

enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}
