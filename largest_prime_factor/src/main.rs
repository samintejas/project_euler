fn main() {
    let input = 600851475143i64;
    let sqrt: i64 = match input.checked_isqrt() {
        None => panic!("Input is not a valid number"),
        Some(i) => i,
    };

    println!("Square root of input is {}", sqrt);

    let mut prime_pointer = 2;
    let mut current_remaining = input;
    let mut remaining;
    let mut primes: Vec<i64> = Vec::new();

    loop {
        remaining = current_remaining / prime_pointer;

        if current_remaining % prime_pointer != 0 {
            prime_pointer = prime_pointer + 1;
            continue;
        } else {
            current_remaining = remaining;
            primes.push(prime_pointer);
        }

        if remaining == 1 || prime_pointer > sqrt {
            break;
        }
    }

    println!("primes are {:?}", primes);
}
