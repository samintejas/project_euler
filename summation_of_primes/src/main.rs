fn main() {
    let mut prime_list: Vec<u64> = Vec::new();
    let mut pointer = 1u64;
    while pointer < 2000000 {
        pointer += 1;
        let mut found_non_prime = false;
        for prime in prime_list.iter() {
            if pointer % prime == 0 {
                found_non_prime = true;
                break;
            }
        }
        if !found_non_prime {
            prime_list.push(pointer);
        }
    }
    let mut sum = 0u64;
    for prime in prime_list.iter() {
        sum += prime;
    }
    println!("sum: {}", sum)
}
