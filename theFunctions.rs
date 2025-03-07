// fn hello() {
//     println!("Warap People");
// }

fn main() {
    // hello();
    // tell_height(210);
    // human_id("Trevor Arapu", 30, 220.2, "trevorarapu2@gmail.com")

    // let _x: i32 = {
    //     let price: i32 = 5;
    //     let qty: i32 = 10;
    //     price*qty
    // };
    // println!("Result is: {}", _x);

    // add(4,6)
    // let y: i32 = add(4,6);
    // println!("The value of y is {}", y);
    // println!("The value from the function is {}.", add(4,6));

    let weight: f64 = 70.0;
    let height: f64 = 1.82;
    let bmi = calc_bmi(weight, height);
    println!("Your BMI is {:.3}", bmi)
}

// fn tell_height(height: u32) {
//     println!("My height is {}", height);
// }

// fn human_id(name: &str, age: u32, height: f32, email: &str) {
//     println!("My name is {}, email {} plus am {} years old and about {}cm tall.", name, email, age, height)
// }

// fn add(a: i32, b: i32) -> i32 {
//     a + b
// }

// BMI = height(kg)/height(m)^2

fn calc_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}