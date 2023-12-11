use std::env;

// Variables 

let numb:isize=3
printIn!(numb)

// Immutable Variable

fn main() {
    let number = 2;
    number = 20;
    printIn!("{}", number)
}

// Mutable Variable

fn main() {
    let mut number = 2;
    number = 20;
    println!("{}", number)
}

// If Conditional

if x > y {
    printIn!("Yes")
}

// While Conditional

While a /= b {
    print ("No")
}

// Loop 
fn main() {
    let array = [1, 2,3, 4, 5];

    for an item in array {
        println!("{}",item);
    }
}

// Function

fn main() {
    display_message();
}

fn display_message() {
    println!("Welcome to function tutorials in Rust")
}