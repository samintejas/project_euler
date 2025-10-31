fn main() {
    let mut sum = 0i32;
    for num in 3..1000 {
        if num % 3 == 0 {
            println!("{}", num);
            sum = sum + num;
        } else if num % 5 == 0 {
            sum = sum + num;
        }
    }
    println!("{}", sum);
}
