use std::io;

fn get_number() -> u64 {
    let mut input: String = String::new();

    println!("Please enter your number: ");

    io::stdin().read_line(&mut input).expect("Failed to read number");

    let number: u64 = input.trim().parse().unwrap();
    number
}

fn sum_digits(num: i64) -> i64 {
    let mut sum = 0;
    let abs_num = num.abs();

    // Convert the absolute value to a string to easily iterate through digits
    let num_str: String = abs_num.to_string();

    for char in num_str.chars() {
        sum += (char as u8 - b'0') as i64

    }

    sum
}

fn find_harshad(nth: u64) -> i64{
    let harshad: i64;
    let mut count: i64  = 1;
    let mut nth_count: u64 = 0;
    loop{
        if count % sum_digits(count) == 0{
            nth_count += 1;
            if nth_count == nth{
                harshad = count;
                break;
            }
        }
        count += 1;
    }
    harshad
}

fn main() {
    let number: u64 = get_number();
    let harshad: i64 = find_harshad(number);
    println!("The {} harshad number is {}", number, harshad)
}