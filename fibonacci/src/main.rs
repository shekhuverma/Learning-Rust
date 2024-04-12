use std::io;
fn main() {
    println!("To generate n fibonacci numbers!");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Error Reading the number!");
    let mut n: i32 = n.trim().parse().expect("Enter a valid integer");

    let mut i = 0;
    let mut j = 1;
    let mut c = 0;
    while n != 0 {
        println!("{i}");
        c = i + j;
        i = j;
        j = c;
        n -= 1;
    }
}

// Link - https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html
