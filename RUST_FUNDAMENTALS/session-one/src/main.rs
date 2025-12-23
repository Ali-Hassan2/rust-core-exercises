mod basics;
mod ownership;
mod struct_enum;
mod vectors_hash_maps;
mod error_handling;

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

    struct_enum::structs::run();
    struct_enum::enums::run();

    vectors_hash_maps::vecotrs_h::run();

    error_handling::error_handling_imp::run();
}
