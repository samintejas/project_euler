fn main() {
    'outer: loop {
        for a in 0..1000 {
            for b in 0..1000 {
                for c in 0..1000 {
                    if a + b + c == 1000 && a * a + b * b == c * c && (a < b) && (b < c) {
                        println!("{} * {} * {} = {}", a, b, c, a * b * c);
                        break 'outer;
                    }
                }
            }
        }
    }
}
