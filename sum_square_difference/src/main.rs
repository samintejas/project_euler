fn main() {
    let limit = 100u32;
    let mut count = 0u32;
    let mut count_square_sum = 0u32;
    let mut count_sum = 0u32;

    while count < limit {
        count = count + 1;
        let count_square = count * count;
        count_square_sum = count_square_sum + count_square;
        count_sum = count_sum + count;
    }

    let count_sum_squared = count_sum * count_sum;
    let final_out = count_sum_squared - count_square_sum;
    println!("output {}", final_out)
}
