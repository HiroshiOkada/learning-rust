fn main() {

    let mut count = 1;

    loop {
        if count > 10 {
            break;
        }
        println!("{}", count);
        // Increment count
        count += 1;
    }
}
