
//contants

fn main() {
    println!("Hello, world!");

    let x = 5;
    const Y: i32 = 10;

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", Y);
    println!("The value of PI is: {:.2}", PI);
    println!("The value of THREEHOURSINSECONDS is: {}", THREEHOURSINSECONDS);
}

// global constants
const PI: f32 = 3.14159;
const THREEHOURSINSECONDS: u64 = 60*60*3;