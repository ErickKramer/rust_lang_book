// Program that calculates the area of a rectangle

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    // Associated function that is not a method!
    // Self is an alias for the type that appears after the impl keyword
    // An associated method is called by using the :: syntax
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area_noob(width1, height1)
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuples((width1, height1))
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect1)
    );

    println!("Rect1 is {:?}", rect1);
    dbg!(&rect1);

    println!(
        "--> The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    // --- Check if rectangles fit
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(3);
    println!("Square {:?}", square);
}

fn area_noob(width: u32, height: u32) -> u32 {
    width * height
}


fn area_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
