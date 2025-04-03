fn main() {
    let key_char: char = 'x';

    // get the input text from the user
    let mut input_text = String::new();
    println!("Please input some letters!");
    std::io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read input text!");
    let input_text = input_text.trim();
    println!("You entered: ({})", input_text);

    // print the length of the string
    println!("leghth: {}", input_text.len());

    // print the reversed string
    let mut reversed_text = String::new();
    for c in input_text.chars() {
        reversed_text.insert(0, c);
    }
    println!("Reversed: ({})", reversed_text);

    // check if the key_char is included in the input text
    if input_text.contains(key_char) {
        println!("The key character ({}) is included in the input text.", key_char);
    } else {
        println!("The key character ({}) is not included in the input text.", key_char);
    }

    // print the input text in uppercase
    println!("{}", input_text.to_uppercase())

}
