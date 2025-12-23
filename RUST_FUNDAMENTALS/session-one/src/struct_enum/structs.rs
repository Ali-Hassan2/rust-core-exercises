// - A struct is a custom data type that groups related values together into a single unit with named fields.

pub fn run(){

    let user = User{
        username: String::from("Ali Hassan"),
        email:String::from("alihassan@mail.com"),
        password:String::from("brohAlien"),
        token:String::from("c5AsQBX5/iNefoyAj0TRkBF2sW/2w+jpnXV2ipKbCR4="), //  generated this using openssl
        bears:String::from("level-1 level-2 level-3"),
    };

    println!("{:?}",user);

}
// #[derive(Debug] automatically implements the Debug trait, allowing this struct/enum to be printed using {:?}
#[allow(dead_code)]
#[derive(Debug)]
struct User{
    username:String,
    email:String,
    password:String,
    token:String,
    bears:String
}

