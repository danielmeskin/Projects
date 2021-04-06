use std::io;
//fn validate(mut card_number: String) -> bool {
//    let check_digit=card_number.pop();
//    let check_digit=check_digit.expect("Failed to extract check digit.").to_digit(10).unwrap();
//    let card_list=card_number.chars();
//    let mut check_list = card_number
//    .chars()
//    .step_by(2)
//    .map(|num| {
//        let num = num.to_digit(10).unwrap() * 2;
//        if num >= 10 {
//            num-9
//        } else {
//            num
//        }
//    })
//    .collect::<Vec<_>>();
//    for number in card_list.step_by(2).skip(1) {
//        let number=number.to_digit(10).unwrap()*2;
//        check_list.push(number);
//    }
//    let mut check_sum = 0;
//    for number in check_list {
//        check_sum += number;
//    }
//    let mut valid = false;
//    if (check_sum+check_digit)%10 == 0 {
//        valid = true;
//    }
//    valid
//}
fn string_to_ints(string: &str) -> Vec<u32> { // This is not my code - from jeffcarp/luhn-rs
    let mut numbers = vec![];
    for c in string.chars() {
        let value = c.to_string().parse::<u32>();
        match value {
            Ok(v) => numbers.push(v),
            Err(e) => println!("error parsing number: {:?}", e),
        }
    }
    numbers
}

fn validate(card_number: &str) -> bool { // This is not my code - from jeffcarp/luhn-rs
    let mut numbers = string_to_ints(card_number);
    numbers.reverse();
    let mut is_odd: bool = true;
    let mut odd_sum: u32 = 0;
    let mut even_sum: u32 = 0;
    for digit in numbers {
        if is_odd {
            odd_sum += digit;
        } else {
            even_sum += digit / 5 + (2 * digit) % 10;
        }
        is_odd = !is_odd
    }

    (odd_sum + even_sum) % 10 == 0
}
fn main() {
    println!("Enter credit card number: ");
    let mut card = String::new();
    io::stdin()
        .read_line(&mut card)
        .expect("Failed to read line");
    card=card.trim().to_string();
    if validate(&card) {
        println!("The card is valid!");
    } else {
        println!("The card is invalid!");
    }
}