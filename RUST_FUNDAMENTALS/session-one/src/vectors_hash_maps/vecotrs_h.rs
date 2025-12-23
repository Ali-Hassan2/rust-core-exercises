use std::collections::HashMap;

pub fn run(){

    let mut vector_one:Vec<i32> = vec![1,2,3,4];
    vector_one.push(5);

    for itr in &vector_one{
        println!("Vector Items: {}",itr);
    }

    let mut hash_map_one:HashMap<String, i32> = HashMap::new();
    hash_map_one.insert(String::from("Ali"),10);
    hash_map_one.insert(String::from("Hassan"),20);

    for (name,score) in &hash_map_one{
        println!("Name - {}, Score - {}: ",name,score)
    } 

}