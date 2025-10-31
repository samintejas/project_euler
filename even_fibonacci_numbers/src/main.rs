fn main() {
    let mut pointer_one = 1i64;
    let mut pointer_two = 2i64;
    let mut current_sum;
    let mut stack = Vec::new();

    stack.push(pointer_one);
    stack.push(pointer_two);
    loop {
        current_sum = pointer_one + pointer_two;
        if current_sum >= 4000000 {
            break;
        }
        pointer_one = pointer_two;
        pointer_two = current_sum;
        stack.push(pointer_two);
    }
    println!("fibonacci sequence up to four million: {:?}", stack);
    current_sum = 0i64;
    for term in &stack {
        if term % 2 == 0 {
            current_sum += term;
        }
    }
    println!(
        "sum of even fibonacci terms up to four million: {}",
        current_sum
    );
}
