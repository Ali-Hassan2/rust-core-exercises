pub fn run(){
    let variable_one = 10;
    println!("variable_one = {}",variable_one);

    // immutable variable
    let mut immutable_variable = 20;
    println!("Before: y = {}",immutable_variable);
    immutable_variable += 5;
    println!("After incrementing +5: {}",immutable_variable);

    // constant 
    const PI: f64 = 3.1415;
    println!("PI Value: {}",PI)

}