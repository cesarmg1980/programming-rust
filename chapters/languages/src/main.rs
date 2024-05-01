fn main() {
    // Get the command line arguments as a vector of strings 
    let languages: Vec<String> = std::env::args().skip(1).collect();
    for l in languages {
        println!("{}: {}", l, if l.len() % 2 == 0 {
            "functional"
        } else {
            "imperative"
        });
    }
}
