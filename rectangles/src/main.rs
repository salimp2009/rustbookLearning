use std::fmt::Display;

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[allow(dead_code)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Associate functions
impl Rectangle {
    // methods takes self as input
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Self) -> bool {
        self.width >= other.width && self.height >= other.height
    }
    fn can_hold2(&self, other: &Self) -> bool {
        self >= other
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
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
        height: 25,
    };

    let rect2_area = area3(&rect2);
    println!("rect2_area : {rect2_area}");
    println!("rectangle_2 : {:#?}", rect2);

    println!("rect2 area via Rectangle method: {}", rect2.area());
    println!("rect2 after impl Display {rect2}");
    println!("check width method width > 0: {}", rect2.width());
    let rect3 = Rectangle {
        // width: dbg!(15),
        width: 10,
        height: 20,
    };

    let rect4 = Rectangle {
        // width: dbg!(15),
        width: 25,
        height: 35,
    };

    println!("rect2 can hold rect3 ? => {}", rect2.can_hold(&rect3));
    println!("rect2 can hold rect4 ? => {}", rect2.can_hold2(&rect4));

    let square1 = Rectangle::square(5);
    println!("square: {square1}");
}
