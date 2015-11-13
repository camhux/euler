fn main() {
    let mut container: [usize; 2] = [0, 1];
    let mut sum: usize = 0;

    while container[1] < 4_000_000 {
        let next = container[0] + container[1];
        container[0] = container[1];
        container[1] = next;
        if container[1] % 2 == 0 {
            sum += container[1];  
        }
    }

    println!("Sum below 4 million is {}", sum);
}
