// fn double_number(n: i32) -> i32 {
//     n * 2
// }

// fn double_or_nothing(n: i32) -> i32 {
//     if n > 0 {
//         return double_number(n);
//     }

//     0
// }

// fn greet(s: String) {
//     println!("Hello {s}")
// }

// fn greet_borrow(s: &String) {
//     println!("Hello {s}")
// }

// fn greet_borrow_mut(s: &mut String) {
//     *s = format!("Hello {s}");
//     println!("Hello {s}")
// }

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Std in is not working");

    input.trim().to_string()
}

fn main() {
    // let n = 6;
    // println!("doubled number greater than 0: {}", double_or_nothing(n));
    // let mut name = "Vitaliy".to_string();
    // greet_borrow(&name);
    // greet_borrow_mut(&mut name);
    // greet(name);

    let input = read_line();
    println!("You have typed: {input}")

}
