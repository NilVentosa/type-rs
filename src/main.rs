fn main() {
    if let Err(e) = type_rs::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
