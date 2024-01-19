use std::env;

mod texparse;

fn main() {
    let args: Vec<String> = env::args().collect();
    let beamer = texparse::BeamerFile::read(&args[1]).unwrap();
    println!("{}", beamer.parse().to_string());
}
