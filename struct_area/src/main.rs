#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        (self.width + self.height) * 2
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {

    let rect1 = Rectangle { width: 30, height: 50};
    let rect2 = Rectangle { width: 10, height: 40};
    let rect3 = Rectangle { width: 60, height: 45};

    // using a function to calculate the area
    println!("The area of rectangle rect1 is {} square pixles.", area(&rect1));
    
    // using the struct impl (area) to calculate the area/perimeter
    println!("The area of rectangle rect1 via the impl method is {} square pixles.", rect1.area());
    println!("The perimeter of rectangle rect1 is {} pixles.", rect1.perimeter());
    println!("values of rectangle rect1 are {:#?}", rect1);

    // using the can_hold function
    println!("Can rect1 hold rect2? : {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? : {}", rect1.can_hold(&rect3));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
