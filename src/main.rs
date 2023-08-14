mod year_2023;

// In order for rust-analyzer to work, I need to treat
// Each nested directory and files as a module, otherwise
// for the moment, seems that rust-analyzer wouldn't work for
// standalone files and I wouldn't get intellisense.
fn main() {
    println!("{:?}", "Rust algorithms and data structures");
}
