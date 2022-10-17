//enumeraciones
#[derive(Debug)]
enum Enumeration {
    //todo!("Fix enum definition so code compiles");
    One,
    Two,
    A,
    B,
}

// Declare Car struct to describe vehicle with four named fields
// struct Car {
//     color: String,
//     transmission: Transmission,
//     convertible: bool,
//     mileage: u32,
// }

fn main(){
    let varible01 = 1;
    let varible02 = "hola".to_string();
    let mut varible_mut01: u32 = 12;
    let mut varible_mut02: bool = false;

    println!("-------------------------------------------------------------------");
    println!("VARIABLES");
    println!("varible01: {}, varible02: {}, varible_mut01: {}, varible_mut02: {}"
        , varible01, varible02, varible_mut01, varible_mut02);

    println!("mutando las varibles varible_mut01, varible_mut02");
    varible_mut01 = varible_mut01 + 4;
    varible_mut02 = true;
    
    println!("varible01: {}, varible02: {}, varible_mut01: {}, varible_mut0: {}"
        , varible01, varible02, varible_mut01, varible_mut02);
    println!("-------------------------------------------------------------------\n");

    println!("TUPLAS");
    let tuple_01 = ("hola", 5, 7.2, ":)");
    println!("la tupla_01 tiene: valor 1 = {}, valor 2 = {}, valor 3 = {}, valor 4 = {}"
        , tuple_01.0, tuple_01.1, tuple_01.2, tuple_01.3);
    println!("la tupla_01 imp completa {:?}", tuple_01);
    println!("-------------------------------------------------------------------\n");

    println!("ENUMERACION");
    let enum_01 = Enumeration::One;
    let enum_02 = Enumeration::Two;
    let enum_03 = Enumeration::A;
    let enum_04 = Enumeration::B;

    println!("enum_01 = {:#?}, enum_02 = {:#?}, enum_03 = {:#?}, enum_04 = {:#?}"
        , enum_01, enum_02, enum_03, enum_04);
    println!("-------------------------------------------------------------------\n");
}