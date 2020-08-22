#[derive(Debug)]
struct Rec {
    width: u32,
    height: u32,    
}

fn main() {
    let rec1 = Rec {
        width: 20,
        height: 25,
    };
    
    println!("the area is {}", area(&rec1));
}

fn area(rectangle: &Rec) -> u32 {
    rectangle.width * rectangle.height
}
