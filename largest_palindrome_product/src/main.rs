fn main() {
    let mut mult_left = 999;
    let mut mult_right = 999;
    let mut switch = true;
    let mut largest_product = 0;

    while mult_left != 0 && mult_right != 0 {
        if switch {
            mult_left = mult_left - 1;
        } else {
            mult_left = 999;
            mult_right = mult_right - 1;
            switch = !switch;
        }

        if mult_left == 1 {
            switch = !switch
        }

        let product = mult_left * mult_right;
        if check_palindrome(product) {
            println!(
                "Largest palindrom is {} and the components are {} * {}",
                product, mult_left, mult_right
            );
            if product > largest_product {
                largest_product = product;
            }
        }
    }

    println!("Largest product is {}", largest_product);

    // while mult_left != 0 && mult_right != 0 {
    //     println!("checking palindrom from {} {}", mult_left, mult_right);
    //
    //     if switch {
    //         mult_left = mult_left - 1;
    //         switch = !switch;
    //     } else {
    //         mult_right = mult_right - 1;
    //         switch = !switch;
    //     }
    //
    //     let product = mult_left * mult_right;
    //     if check_palindrome(product) {
    //         println!(
    //             "Largest palindrom is {} and the components are {} * {}",
    //             product, mult_left, mult_right
    //         );
    //         break;
    //     }
    // }
}

fn check_palindrome(num: i32) -> bool {
    let straight: Vec<char> = num.to_string().chars().collect();
    let reverse: Vec<char> = straight.iter().rev().cloned().collect();
    for index in 0..(straight.len() - 1) {
        if straight[index] != reverse[index] {
            return false;
        }
    }
    return true;
}
