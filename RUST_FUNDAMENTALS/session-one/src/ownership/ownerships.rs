pub fn run(){
   
    // In Rust, ownership is a set of rules that governs how memory is managed. It’s one of Rust’s core features that allows memory safety without a garbage collector. Every value in Rust has a single owner, and Rust enforces rules at compile time to prevent invalid memory access.

    let string_one = String::from("rust is amazing");
    let string_two = string_one; // string assigned to string_two now string_one is memory free

    // println!("{}",string_one); // this will cause an Error
    println!("{}",string_two);
    let string_three:String = String::from("rust is great");
    let string_length:usize = calculate_length(&string_three);
    println!("Length of {} String three is {}:",string_three,string_length)

}

fn calculate_length(string_parameter:&String)-> usize{
    string_parameter.len()
}