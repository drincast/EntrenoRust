//se debe llamar la definicion de HasMap
use std::collections::HashMap;

fn main(){
    //definicion de HasMap
    let mut hash_map: HashMap<String, String> = HashMap::new();

    hash_map.insert("key1".to_string(), "value1".to_string());
    hash_map.insert(String::from("key2"), String::from("value2"));
    hash_map.insert(String::from("key3"), String::from("value3"));

    println!("key1: {:?}, key2: {:?}, key3: {:?}", hash_map.get("key1"), hash_map.get("key2")
        , hash_map.get("key3"));

    hash_map.remove("key3");

    println!("key1: {:?}, key2: {:?}, key3: {:?}", hash_map.get("key1"), hash_map.get("key2")
        , hash_map.get("key3"));
}