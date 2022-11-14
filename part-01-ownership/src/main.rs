#[derive(Debug)]
enum OptionU32 {
    OK(u32),
    None,
}

#[derive(Debug, Clone)]
struct Car {
    length_in_meters: f32,
    width_in_meters: f32,
    height_in_meters: f32,
}

type Temperature = u32; // alias

#[derive(Debug)]
struct Temp(u32); // Tuple with one value

/// Display a Car
///
/// This function displays a car (value moved)
fn display_car(car: Car) {
    println!(
        "Car [{}, {}, {}]",
        car.length_in_meters, car.width_in_meters, car.height_in_meters
    );
}

/// Displays a Car by reference
///
/// This function uses a car by reference and displays it
fn display_car_by_reference(car: &Car) {
    println!(
        "Car by ref [{height}, {width}, {length}]",
        length = car.length_in_meters,
        width = car.width_in_meters,
        height = car.height_in_meters
    );
}

/// Explains ownership with simple types
///
/// Simple types are copied unlike complex types.
fn ownership_with_simple_types() {
    let inside_temp: Temperature = 25;
    let outdoor_temp = inside_temp;

    println!(
        "Temp interne = {}, temp externe = {}",
        inside_temp, outdoor_temp
    );

    let mut inside_hygrometry: f64 = 0.84; // in percent
    let outdoor_hygrometry = &inside_hygrometry;

    // inside_hygrometry = 0.88; // Error inside_hygrometry is borrow with outdoor_hygrometry

    println!("Hygrometry = {}, {}", inside_hygrometry, outdoor_hygrometry);

    let outdoor_hygrometry = inside_hygrometry; // Shadowing (outdoor_hydrometry replace old variable)
    inside_hygrometry = 0.88;

    println!("Hygrometry = {}, {}", inside_hygrometry, outdoor_hygrometry);
}

/// Explains ownership with complex types
///
/// Structs, Enums are moved by default
fn ownership_with_complex_types() {
    let my_car = Car {
        length_in_meters: 3.25,
        width_in_meters: 1.75,
        height_in_meters: 0.95,
    };

    let neighbour_car = my_car;
    let friend_car = neighbour_car.clone(); // Ok Copy

    // println!("My car = {:?}", my_car); // Error my_car is moved to neighbour_car
    println!(
        "My car = {:?}, friend_car = {:?}",
        neighbour_car, friend_car
    );

    display_car(neighbour_car);

    // println!("My car = {:?}", neighbour_car); // Error neighbour_car is moved to consume_car()

    display_car_by_reference(&friend_car);

    let my_option = OptionU32::None;

    let mut other_option = my_option;

    // println!("my_option = {:?}", my_option); // Error my_option is moved to other_option

    println!("my_option = {:?}", other_option);

    other_option = OptionU32::OK(10);

    println!("my_option = {:?}", other_option);

    let outdoor_temp = Temp(25);
    let inside_temp = outdoor_temp;

    println!("Inside temp = {:?}", inside_temp);

    // println!("Outdoor temp = {:?}", outdoor_temp); // Error outdoor_temp is moved to inside_temp
}

fn ownership_loops() {
    let mut car = Car {
        height_in_meters: 0.75,
        length_in_meters: 2.50,
        width_in_meters: 3.0,
    };

    for index in 1..=10 {
        // let temp_car = car; // Error car is moved at first iteration
        let temp_car = &mut car;
        // let another_temp_car = &car; // Error, already borrowed (temp_car)

        temp_car.height_in_meters += 0.02;

        println!("Car [iter={}] = {:?}", index, temp_car);
    }
}

fn main() {
    ownership_with_simple_types();
    ownership_with_complex_types();
    ownership_loops();
}
