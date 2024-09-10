#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let mut rec1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of rec1: {}", rec1.area());

    let ref_rec1 = &mut rec1;
    println!("Height: {}, Width: {}", rec1.height, rec1.width);

    println!("Rec1: {:?}", rec1);
}

