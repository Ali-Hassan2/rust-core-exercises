use std::time::Duration;
use tokio::time::sleep;

pub async fn run(){
    println!("Asyn task is under execution.");
    sleep(Duration::from_secs(2)).await;
    println!("Task completed the data is here.")
}