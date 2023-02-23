enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Inside call method");
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    println!("Hello, world!");

    let m = Message::Write(String::from("Hello"));
    m.call();

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{five:?}");
    println!("{six:?}");
    println!("{none:?}");
}
