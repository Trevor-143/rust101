// 1
// each value in rust has a variable thats its owner

// fn main() {
//     let s1 = String::from("RUST");
//     let len = cal_length(&s1);
//     println!("The length of {} is {}", s1, len)
// }

// fn cal_length(s: &String) -> usize {
//     s.len()
// }

// 2
// only one owner at a time

// fn main() {
//     let s1 = String::from("JAVA");
//     let s2 = s1;
//     // println!("{}", s1)
//     println!("{}", s2)
// }

// side
// fn main() {
//     let s1 = String::from("JAVASCRIPT");
//     let _s2 = s1.clone();
//     println!("{}", s1);
//     println!("{}", _s2)
// }

// 3
// when owner goes out of scope the value will be dropped

fn main() {
    let s1 = String::from("PHP");
    let len = cal_len(&s1);

    println!("Length of {} is {}", s1, len);
}

fn printLost(s: &String) {
    println!("{}", &s1);
}

fn cal_len(s: &String) -> usize {
    s.len()
}