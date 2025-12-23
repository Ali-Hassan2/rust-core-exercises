pub fn run(){

    let some_number = Some(10);

    let no_number: Option<i32> = None;

    println!("Some number: {:?}",some_number);

    println!("No number: {:?}",no_number);

    let result = divide(10,2);
    match result{
        Ok(v) => println!("Division Successfull: {}",v),
        Err(e) => println!("cannot Perform Division: {}",e),
    }

}

fn divide(number_one:i32,number_two:i32)-> Result<i32,String>{

    if number_two == 0{
        Err(String::from("Result can be infinite cannot perform division."))
    }
    else{
        Ok(number_one/number_two)
    }

}