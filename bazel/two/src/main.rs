fn main() {
    println!("{}", greet("Alice"));
    println!("{}", greet_number("Bob", 2, 3));
}

pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub fn greet_number(name: &str, a: isize, b: isize) -> String {
    format!("Hello, {}! {}+{}={}", name, a, b, crate_one::add(a, b))
}
