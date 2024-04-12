use core::num;
use std::io;
fn main() {
    println!("For Celius to Fahrenhiet press 1");
    println!("For Fahrenhiet to Celius press 2");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    // To remove the trailling white sapce
    let choice = choice.trim();

    if choice == "1" {
        println!("Converting Celcius to fahrenhiet!");
        let input: f32 = input();
        println!("Entered temperature is {input}");

        let result: f32 = (input * 9.0 / 5.0) + 32.0;

        println!("Temperature in celcius = {result}");
    } else if choice == "2" {
        println!("Converting fahrenhiet to Celcius!");
        let input: f32 = input();
        println!("Entered temperature is {input}");

        let result: f32 = (input - 32.0) * (5.0 / 9.0);

        println!("Temperature in celcius = {result}");
    } else {
        println!("Invalid Choice");
    }
}

fn input() -> f32 {
    let mut number = String::new();

    println!("Enter the Temperature :");

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line!");

    let n: f32 = number.trim().parse().expect("not a valid number");
    // By not adding semicolon at the end, the function is returning n
    // we can also do it implicitly by using return n
    n
}
