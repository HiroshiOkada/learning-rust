// 
trait Describe {
    fn describe(&self) -> String;
}

struct Book {
    title: String,
    author: String
}

struct Movie {
    title: String,
    director: String
}

impl Describe for Book {
    fn describe(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

impl Describe for Movie {
    fn describe(&self) -> String {
        format!("{} directed by {}", self.title, self.director)
    }
}

// Describe trit 境界を持つジェネリック関数を定義
fn print_description<T: Describe>(item: T) {
    println!("{}", item.describe());
}

fn main() {
    let my_book = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
    };

    let my_movie = Movie {
        title: String::from("Rust in Motion"),
        director: String::from("Cargo B. Crate"), 
    };

    print_description(my_book);
    print_description(my_movie);
}