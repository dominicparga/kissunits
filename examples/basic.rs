use kissunits::{
    distance::{Kilometers, Meters},
    time::{Hours, Seconds},
};

fn main() {
    // use the struct directly
    let m = Meters(72_000.0);
    // or use the factory-function 'new(...)'
    let h = Hours::new(2.0);

    // compile-error since resulting unit is not clear
    // println!("{}", m / h); // ERROR

    // prints 36 km/h
    let km = Kilometers::from(m);
    println!("{} / {} = {}", km, h, km / h);

    // prints 10 m/s
    let s = Seconds::from(h);
    println!("{} / {} = {}", m, s, m / s);
}
