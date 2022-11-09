//se debe llamar la definicion de HasMap
use std::collections::HashMap;
use std::convert::TryInto;
// use hashbrown::HashMap;

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

    println!("--------------------------------------------------------------\n");
    println!("LOOP");

    let mut id_key : i8 = 1;
    let str_key : String = String::from("key");
    loop {
        if id_key >= 4 {
            break;
        }

        let index = format!("{}{}", str_key, &id_key.to_string());

        println!("{}: {:?}", index, &hash_map.get(&index));
        id_key = id_key + 1;
    }

    println!("--------------------------------------------------------------\n");
    println!("WHILE");

    let number_items = hash_map.keys().len();
    id_key = 0;
    println!("number_items: {}", number_items);
    while id_key < number_items.try_into().unwrap() {
        id_key = id_key + 1;
        let index = format!("{}{}", str_key, &id_key.to_string());
        println!("{}: {:?}", index, &hash_map.get(&index));
    }

    println!("--------------------------------------------------------------\n");
    println!("FOR");

    for item in &hash_map {
        println!("{:?}", item);
    }

    println!("\n");

    for (key, value) in &hash_map {
        println!("{key}: \"{value}\"");
    }
}