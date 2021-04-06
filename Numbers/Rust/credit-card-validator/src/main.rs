use std::io;

fn validate(mut card_number: String) -> bool {
    let check_digit=card_number.pop();
    let check_digit=check_digit.expect("Failed to extract check digit.").to_digit(10).unwrap();
    let card_list=card_number.chars();
    let mut check_list=vec![];
    for number in card_list.clone().step_by(2) {
        let mut number=number.to_digit(10).unwrap()*2;
        if number >= 10 {
            number = (number-10)+1;
        }
        check_list.push(number);
    }
    for number in card_list.step_by(2).skip(1) {
        let number=number.to_digit(10).unwrap()*2;
        check_list.push(number);
    }
    let mut check_sum = 0;
    for number in check_list {
        check_sum += number;
    }
    let mut valid = false;
    if (check_sum+check_digit)%10 == 0 {
        valid = true;
    }
    valid
}
fn main() {
    println!("Enter credit card number: ");
    let mut card = String::new();
    io::stdin()
        .read_line(&mut card)
        .expect("Failed to read line");
    card=card.trim().to_string();
    if validate(card) {
        println!("The card is valid!");
    } else {
        println!("The card is invalid!");
    }
}