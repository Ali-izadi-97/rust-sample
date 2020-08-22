#[derive(Debug)]
struct Numbers {
    num1: u32,
    num2: u32,
}

impl Numbers {
    fn prod(&self) -> u32 {
        self.num1 * self.num2
    }

    fn sm(&self) -> u32 {
        self.num1 + self.num2
    }
}

fn main() {
    let obj = Numbers {
        num1: 20,
        num2: 30
    };

    println!("object is {:?}", obj);
    println!("prod is {} and sum is {}", obj.prod(), obj.sm());
}
