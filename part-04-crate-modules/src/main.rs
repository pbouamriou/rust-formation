mod vehicules;

use crate::vehicules::*;

fn main() {
    let my_potatoe = Car {
        weigth_in_kilo_grams: 1200.0,
        length_in_meters: 2.54,
        width_in_meters: 1.25,
    };
    println!("Hello, world!, {:?}", my_potatoe);
}
