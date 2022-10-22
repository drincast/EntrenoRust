fn main(){
    let array_number01 = [1, 2, 3, 4, 5];
    let mut array_number02 = [1; 5];

    println!("Arrays in RUST !!!");
    println!("array_number01 = [1, 2, 3, 4, 5]; = {:?}", array_number01);
    println!("mut array_number02 = [1; 5]; = {:?}", array_number02);

    array_number02[0] = 6;
    array_number02[1] = 7;
    array_number02[2] = 8;
    array_number02[3] = 9;
    array_number02[4] = 10;

    println!("array_number02 set 6 to 10; = {:?}", array_number02);
    println!("--------------------------------------------------\n");

    println!("Vectors in RUST !!!");
    //NOTE: hay que declararlo mutable para que deje agregar valores
    let mut vec01 = Vec::<u32>::new();
    let vec02 = vec![4, 5, 6];
    let vec03 = vec![3; 3];

    println!("mut vec01 = {:?}", vec01);
    println!("vec02 = {:?}", vec02);
    println!("vec03 = {:?}\n", vec03);

    println!("agregamos valores a vec01 usando método push");
    vec01.push(1);
    vec01.push(2);
    vec01.push(3);    
    println!("mut vec01 = {:?}", vec01);
    println!("vec02 = {:?}", vec02);
    println!("vec03 = {:?}", vec03);

    println!("\nusamos en vec01 método pop");
    vec01.pop();
    println!("mut vec01 = {:?}", vec01);

    println!("\nen vec01 agregamos el 3 e imprimimos el tercer elemento de los 3 vectores");
    vec01.push(3);
    println!("vec01[2] = {}, vec02[2] = {}, vec03[2] = {}", vec01[2], vec02[2], vec03[2]);
}