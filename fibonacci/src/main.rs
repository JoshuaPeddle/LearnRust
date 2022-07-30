use std::env;
#[macro_use]
extern crate exec_time;

// fn is_fibonacci(num: u128) -> bool {
//     if num == 0 || num == 1 {
//         return true;
//     }
//     let mut a = 0;
//     let mut b = 1;
//     let mut c = a + b;
//     while c < num {
//         a = b;
//         b = c;
//         c = a + b;
//     }
//     c == num
// }

fn generate_fibonacci(num: u128) -> Vec<u128> {
    let mut a = 0;
    let mut b = 1;
    let mut c = a + b;
    let mut result = vec![a, b];
    while c < num {
        a = b;
        b = c;
        c = a + b;
        result.push(c);
    }
    result
}
#[exec_time]
fn main() {

    let args: Vec<String> = env::args().collect();

    let mut num = 184467440737095516000000000000000000000;
    if args.len() > 1 {
        num = args[1].parse::<u128>().unwrap();
    }
    let numbers = generate_fibonacci(num);
    // print length of numbers
    println!("Found {} Fibonacci numbers. {:?}", numbers.len(), numbers);
}
