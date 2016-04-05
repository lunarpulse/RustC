use std::io::Write;
use std::str::FromStr;

fn main() {
    println!("Hello, world!");
    //println!("{:?}",gcd(2*5*17*19, 3*7*9*13*17*27) );

    let mut input_numbers = Vec::new();

    for arg in std::env::args().skip(1){
        input_numbers.push(u64::from_str(&arg)
                        .expect("error parseing argument"));
    }

    if input_numbers.len() ==0 {
        writeln!(std::io::stderr(), "Usage: gcd Number ...").unwrap();
        std::process::exit(1);
    }

    let mut d = input_numbers[0];
    for m in &input_numbers[1..]{
        d = gcd(d,*m);
    }

    println!("the gretest comon divisor of {:?} is {}", input_numbers, d);
}

fn gcd(mut n:u64, mut m: u64) -> u64{
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m; m = n; n = t;
        }
        m = m%n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(2*5*17*19, 3*7*9*13*17*27), 17);
    assert_eq!(gcd(2*3*11*5*7*19, 3*5*17*37*41), 3*5);
}
