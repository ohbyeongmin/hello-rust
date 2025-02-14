fn main() {
    basic();
    debug_trait();
}

fn basic() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        basic_area(width1, height1)
    );

    tuple();

    struct_example();
}

fn basic_area(width: u32, height: u32) -> u32 {
    width * height
}

fn tuple() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        tuple_area(rect1)
    );
}

fn tuple_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn struct_example() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("{:?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        struct_area(&rect1)
    );
}

fn struct_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn debug_trait() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
