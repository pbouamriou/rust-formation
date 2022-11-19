mod macros;

use crate::macros::recurrence;

fn main() {
    let fib = recurrence!( a[n]: u64 = 0, 1; ...; a[n-2] + a[n-1]);
    let test = recurrence!( a[n]: u64 = 0, 1, 3; ...; a[n-2] + a[n-1] + a[n-3]);

    print!("Fib = ");

    for e in fib.take(8) {
        print!("{} ", e)
    }

    print!("\nTest = ");

    for e in test.take(8) {
        print!("{} ", e)
    }

    println!("");
}
