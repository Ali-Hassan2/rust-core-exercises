#[derive(Debug)]
#[allow(dead_code)] // suppresses warnings for unused enum variants
enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

fn check_day() {
    let today = Day::Sunday;
    println!("Today the day is: {:?}", today);
}

#[derive(Debug)]
enum Coin{
    Nickel,
    Penny,
    Dime,
    Quarter(u8),//  unsingned integer 8 bit
}
#[derive(Debug)]
enum Message{
    Quit,
    Move {x: i32, y:i32},
    Write (String),
    changeColor(i32,i32,i32)
}

pub fn run() {
    check_day();
    let c = Coin::Quarter(5);
    println!("Coin: {:?}",c);
    let Message_Quit:Message = Message::Quit;
    let message_one:Message = Message::Write(String::from("Wow signal"));
    let message_two:Message = Message::Move { x: 20, y: 30 };
    let message_to_change_color: Message = Message::changeColor(128,127, 126);
    println!("Write_Message: {:?}, Move_Message: {:?}, change Color: {:?}",message_one,message_two,message_to_change_color);
    let coin:Coin = Coin::Quarter(3);
    println!("Value {}",value_in_cents(coin));
}


fn value_in_cents(coin:Coin ) -> u8{
    match coin{
        Coin::Penny => 1,
        Coin::Nickel =>3,
        Coin::Dime =>5,
        Coin::Quarter(state)=>{
            println!("The quarter value is: {}",state);
            25
        }

    }
}

