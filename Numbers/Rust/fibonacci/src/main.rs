use std::io;
use std::io::Write;
use num_bigint::BigUint;
fn main() {
    print!("What index of the sequence would you like? ");
    io::stdout().flush().unwrap();
    let mut ulimit = String::new();
    io::stdin().read_line(&mut ulimit).expect("Could not read line.");
    let ulimit:u64 = ulimit.trim().parse().expect("Could not parse.");
    let mut a:BigUint=BigUint::from(0_u32);
    let mut b:BigUint=BigUint::from(1_u32);
    let mut sequence:Vec<BigUint> = vec![BigUint::from(1_u32)];
    for _ in 1..ulimit {
        let n: BigUint = a + &b;
        a = b;
        b = n.clone();
        sequence.push(n.clone());
    }
   let last = sequence.last().unwrap().to_string();
   println!("Fibonacci {}: {}", ulimit, last);
}