fn main() {
    let mut index: i64 = 0;
    let mut sum: i64 = 0;
    let mut last_square_searching = true;
    while last_square_searching {
        index = index + 1;
        let square = index * index;
        if (square % 2) != 0 {
            sum = sum + square;
        }
        if index == 580000 {
            last_square_searching = false;
        }
    }

    println!("sum is {}", sum);
}
