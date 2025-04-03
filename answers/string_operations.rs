const KEY_CHAR: char = 'x';

fn main() {
    // get the input text from the user
    let mut input_text = String::new();
    println!("Please input some letters!");
    std::io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read input text!");
    let input_text = input_text.trim();
    println!("You entered: ({})", input_text);

    // print the length of the string
    println!("length: {}", input_text.len());

    // print the reversed string
    let reversed_text: String = input_text.chars().rev().collect();
    println!("Reversed: ({})", reversed_text);

    // check if the KEY_CHAR is included in the input text
    if input_text.contains(KEY_CHAR) {
        println!(
            "The key character ({}) is included in the input text.",
            KEY_CHAR
        );
    } else {
        println!(
            "The key character ({}) is not included in the input text.",
            KEY_CHAR
        );
    }

    // print the input text in uppercase
    println!("{}", input_text.to_uppercase());
    println!("");
}