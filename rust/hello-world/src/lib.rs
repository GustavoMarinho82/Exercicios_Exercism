// &'static is a "lifetime specifier", something you'll learn more about later
pub fn hello() -> &'static str {
    "Hello, World!"
}

pub fn diga(frase: &str) {
    println!("{}", frase);
}