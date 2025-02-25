// fn hello() {
//     println!("Warap People");
// }

fn main() {
    // hello();
    // tell_height(210);
    // human_id("Trevor Arapu", 30, 220.2, "trevorarapu2@gmail.com")

    let _x: i32 = {
        let price: i32 = 5;
        let qty: i32 = 10;
        price*qty
    };
    println!("Result is: {}", _x);
}

// fn tell_height(height: u32) {
//     println!("My height is {}", height);
// }

// fn human_id(name: &str, age: u32, height: f32, email: &str) {
//     println!("My name is {}, email {} plus am {} years old and about {}cm tall.", name, email, age, height)
// }