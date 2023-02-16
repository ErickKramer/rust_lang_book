// Program that calculates the area of a rectangle

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
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
