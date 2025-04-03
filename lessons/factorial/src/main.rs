fn main() {
    let number: u64 = 10;
    let mut factorial: u64 = 1;
    let mut count: u64 = 2;

    while count <= number {
        factorial *= count;
        count += 1;
    } 

    println!("The factorial of {} is {}", number, factorial);
}
