fn main() {
    let mut prime_list: Vec<u32> = Vec::new();
    let mut current = 1u32;

    'outer: loop {
        current = current + 1;

        if prime_list.len() == 10001 {
            break;
        }

        for prime in prime_list.iter() {
            let rem = current % prime;
            if rem == 0 {
                continue 'outer;
            }
        }

        prime_list.push(current);
    }
    println!("{}", prime_list[prime_list.len() - 1]);
}
