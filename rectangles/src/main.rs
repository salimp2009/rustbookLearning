use std::fmt::Display;

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

use std::fmt;
impl Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rectangle; width:{}, height:{}", self.width, self.height)
    }
}

fn area3(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn main() {
    let width = 15;
    let height = 20;
    let area = area(width, height);
    println!("area of rectangle (15x20): {area}");

    let rect = (15, 20);
    let rect_area = area2(rect);
    println!("rect_area (15 x 20): {rect_area}");

    //  let scale = 2;
    let rect2 = Rectangle {
        // width: dbg!(15),
        width: 15,
        height: 20,
    };

    let rect2_area = area3(&rect2);
    println!("rect2_area : {rect2_area}");
    println!("rectangle_2 : {:#?}", rect2);

    println!("rect2 area via Rectangle method: {}", rect2.area());
    println!("rect2 after impl Display {rect2}");
}
