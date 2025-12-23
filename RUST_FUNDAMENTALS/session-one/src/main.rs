mod basics;
mod ownership;

fn main() {
    println!("Hello, world!");
    let variable = "rust";
    println!("Hello {}",variable);
    println!("Hello {} {}",variable,variable);
    println!("Hello {variable}");

    println!("=========== Rust Session One ===========");
    basics::variables::run();
    basics::functions::run();

    ownership::ownerships::run();
}
