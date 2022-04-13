use std::io;

fn main() {
    let mut number = String::new();
    println!("Please input a number smaller than 5.");
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: u32 = number.trim().parse().expect("Not a number!");

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number2 = if number > 5 { 10 } else { 20 };
    println!("The value of number is: {}", number2);

    let mut counter = 0;

    let result = loop {
        counter += 1;
        println!("Now counter is {}", counter);
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
