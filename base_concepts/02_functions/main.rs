#[derive(Debug)]
struct Person {
    age: u8,
    email: String,
    first_name: String,
    last_name: String,
    phone_number: u32,
}

//Al pasar un objeto que no tiene caracteristica de copiar se debe
//pasar por referencia o crear alguna forma para que se pueda usar el 
//objeto despues cuando termine de ejecutar la funcion
fn imp_struct(person: &Person) {
    println!("PersonaWho is he ???? {:?}", person);
}

fn return_strunct(person: Person) -> Person {
    person
}

fn imp_value(value: u32) {
    println!("the value is {}", value);
}

fn return_value(value: u32) -> u32 {
    return value;
}

fn main() {
    let person_b = Person {
        first_name: "Walter Joseph".to_string(),
        last_name: String::from("Kovacs"),
        age: 45,
        phone_number: 555111,
        email: "rorschach@crimebusters.com".to_string(),
    };

    let number: u32 = 50;

    imp_struct(&person_b);
    let person_copy = return_strunct(person_b);
    println!("Watchmen {:?}", person_copy);
    //si se ejecuta la linea siguiente se genera un error
    //^^^^^^^^ value borrowed here after move
    //move occurs because `person_b` has type `Person`, which does not implement the `Copy` trait
    //println!("Watchmen again {:?}", person_b);

    imp_value(number);
    println!("the number is {}", return_value(number));
    let number_copy = return_value(number);
    println!("the return value is {}", number_copy);
}