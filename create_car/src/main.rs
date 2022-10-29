

#[derive(PartialEq, Debug)]
// Declare Car struct to describe vehicle with four named fields
struct Car {
    color: String,
    transmission: Transmission,
    roof: bool,
    age: (Age, u32),
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    // todo!("Fix enum definition so code compiles");
    Manual,
    SemiAuto,
    Automatic,
}

#[derive(PartialEq, Debug)]
enum Age {
    New,
    Used,
}

// Get the car quality by testing the value of the input argument
// - miles (u32)
// Create a tuple for the car quality with the Age ("New" or "Used") and mileage
// Return a tuple with the arrow `->` syntax
fn car_quality (miles: u32) -> (Age, u32) {

    // Declare and initialize the return tuple value
    // For a new car, set the miles to 0
    
    if miles > 0 {
        let quality = (Age::Used, miles); //todo!("Set the `Age` value to \"New\", set the mileage using the `miles` input argument");
        return quality;
    }
    else{
        let quality = (Age::New, miles);
        return quality;
    }
}

// Build a "Car" by using values from the input arguments
// - Color of car (String)
// - Transmission type (enum value)
// - Convertible (boolean, true if car is a convertible)
fn car_factory(color: String, transmission: Transmission, roof: bool, miles: u32) -> Car {

    if car_quality(miles).0 == Age::Used {
        if roof {
            println!("Preparing a used car: {:?}, {}, Hard top, {} miles", transmission, color, miles);
        } else {
            println!("Preparing a used car: {:?}, {}, Convertible, {} miles", transmission, color, miles);
        }
        //println!("{:0?}", car.age.0);
    } else {
        if roof {
            println!("Building a new car: {:?}, {}, Hard top, {} miles", transmission, color, miles);
        } else {
            println!("Building a new car: {:?}, {}, Convertible, {} miles", transmission, color, miles);
        }
    }
    

    // Use the values of the input arguments
    // All new cars always have zero mileage
    let car = Car {color: color
        , transmission: transmission
        , roof: roof
        , age: car_quality(miles)
    }; //todo!("Create an instance of a `Car` struct");

    car
}

fn main() {
    // Create car color array
    let colors = ["Blue", "Green", "Red", "Silver"]; //todo!("Set the enum values: 0 = Blue, 1 = Green, 2 = Red, 3 = Silver");

    // Declare the car type and initial values
    let mut car: Car;
    let mut engine= Transmission::Manual;

    // Order 3 cars, one car for each type of transmission

    // Car order #1: New, Manual, Hard top
    car = car_factory(String::from(colors[2]), engine, true, 0);
    //println!("Car order 1: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.transmission, car.color, car.age.1);

    // Car order #2: Used, Semi-automatic, Convertible
    engine = Transmission::SemiAuto;
    car = car_factory(String::from(colors[0]), engine, false, 100);
    //println!("Car order 2: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.transmission, car.color, car.age.1);

    // Car order #3: Used, Automatic, Hard top
    engine = Transmission::Automatic;
    car = car_factory(String::from(colors[3]), engine, true, 200);
    //println!("Car order 3: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.transmission, car.color, car.age.1);
}
