fn main() {
    let mut sum: usize = 0;

    for n in 1..1000 {
        if n % 5 == 0 ||
           n % 3 == 0 {
            sum += n;
        }
    }
    println!("Sum is {}", sum);
}
