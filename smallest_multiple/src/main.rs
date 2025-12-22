fn main() {
    let mut multiple = 0u32;
    let mut found: bool = false;

    while !found {
        multiple = multiple + 1;
        let mut pointer = 0u32;
        let pointer_divisible = true;

        while pointer_divisible {
            pointer = pointer + 1;
            if multiple % pointer == 0 {
                if pointer == 20 {
                    found = true;
                    break;
                }
                continue;
            } else {
                break;
            }
        }
    }
    println!("multiple is {}", multiple);
}
