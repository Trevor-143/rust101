// compound data types in Rust

fn main() {
    // let numbers: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];
    // println!("The numbers array {:?}", numbers);

    // let mix = ['c', 7, 'a', 'b', false]; // wrong
    // println!("The numbers array {:?}", numbers);

    // let fruits: [&str; 4] = ["Apple", "Banana", "Orange", "Mango"];
    // println!("The fruits array {:?}", fruits);
    // println!("The second fruit in the array {}", fruits[1]);

    // let person: (String, i32, bool) = ("Trevor".to_string(), 32, false);
    // println!("The person is {:?}", person);

    // let number_slices: &[i32] = &[1, 2, 3, 4, 5, 6, 7];
    // println!("The number slices {:?}", number_slices);

    
    // let animal_slices: &[&str] = &["lion", "tiger", "cheetah", "leopard", "jaguar"];
    // println!("The animal slices {:?}", animal_slices);

    let mut stone_cold: String = String::from("Haven, ");
    stone_cold.push_str("is awesome!");

    println!("Trevor says {}", stone_cold);

    let string: String = String::from("Hello, world of 🦀!");
    let slice: &str = &string[0..5];
    println!("The slice is {}", slice);

}