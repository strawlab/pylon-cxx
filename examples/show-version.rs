fn main() {
    println!("{}: {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("{:?}", pylon_cxx::pylon_version());
}
