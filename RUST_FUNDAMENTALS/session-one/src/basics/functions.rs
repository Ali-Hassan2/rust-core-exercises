fn add(a:i32, b:i32) -> i32{
    a + b
}

fn greet_someone(name: &str){
    println!("Hello bro {}",name)
}

pub fn run(){
    println!("Sum: {}",add(10,10));
    greet_someone("Ali Hassan");
}