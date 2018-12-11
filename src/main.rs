struct Ontology {
  name: String,
  version: String,
  author: String,
}

fn main() {
    println!("Hello, world!");
    println!("");
    println!("This is a new project for interacting with bioontologies,");
    println!("written in the Rust programming language.");
    println!("");
    println!("This is an early effort, so stay tuned for planned features.");

    let ont = Ontology {
        name: String::from("VenomKB"),
        version: String::from("2.0.0a"),
        author: String::from("Joseph D. Romano"),
    };

    println!("");
    println!("Ontology name: {}", ont.name);
    println!("Ontology version: {}", ont.version);
    println!("Ontology author: {}", ont.author);
}
