use clap::Parser;

struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}


fn main() {
    let pattern = std::env::args().nth(1).expect("no pattern given");
    println!("Hello, world!");
    println!("{}",pattern);

}
