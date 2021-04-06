use card_validate::Validate;
use std::io;
fn main() {
    println!("Credit card number:");
    let mut credit_card = String::new();
    io::stdin().read_line(&mut credit_card).expect("I/O error");
    credit_card = credit_card.trim().to_string();
    match Validate::from(&credit_card) {
        Ok(result) => println!("Card type is a valid {} credit card.", result.card_type.name()),
        Err(err) => println!("Card is invalid: {:?}", err)
      }
}