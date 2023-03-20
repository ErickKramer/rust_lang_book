/// Find the largest element on a list
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<T, U> {
    x: T,
    y: U,
}

struct Stuff<T> {
    x: T,
}

impl<T> Stuff<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

/// Method only available for Stuff of type f32
impl Stuff<f32> {
    fn meaning(&self) -> f32 {
        self.x.powi(2)
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 5.0, y: 10.0 };
    let mixed = Point { x: 5, y: 10.0 };

    let awesome_stuff = Stuff { x: 5 };

    println!("awesome_stuff.x = {}", awesome_stuff.x());
    let awesome_float: Stuff<f32> = Stuff {x: 2.0};
    println!("awesome_float = {}", awesome_float.meaning());

}
