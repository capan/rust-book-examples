use std::io;

fn main() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of numbers are: {0}, {1}, {2}", x, y, z);
    println!(
        "The value of numbers are: {0}, {1}, {2}",
        tup.0, tup.1, tup.2
    );

    let array = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let length_defined_array: [i32; 5] = [1, 2, 3, 4, 5];
    let same_value_for_each_element = [3; 5];
    println!("The value of array is : {:?}", array);
    println!("The value of months is : {:?}", months);
    println!(
        "The value of length_defined_array is : {:?}",
        length_defined_array
    );
    println!(
        "The value of same_value_for_each_element is: {:?}",
        same_value_for_each_element
    );
    println!(
        "1st element of same_value_for_each_element is: {:?}",
        same_value_for_each_element[0]
    );
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = same_value_for_each_element[index];
    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
