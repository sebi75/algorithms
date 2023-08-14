pub fn random(r: &str) -> String {
    let result = format!("{}{}", r, " random");
    println!("nested module with rust analyzer working");

    result
}

fn main() {
    random("hello");
}
