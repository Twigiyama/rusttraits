use std::any;
use std::fmt;

#[derive(PartialEq, PartialOrd)]
struct Satellite {
    name: String,
    velocity: f64 //miles per second
}

struct SpaceStation { 
name: String,
crew_size: u8,
altitude: u32 //miles
}

trait Description {
    fn describe(&self) -> String {
        format!("an object flying through space")
    }
}

impl Description for Satellite {/* 
    fn describe(&self) -> String {
        format!("the {} flying at {} miles per second!", self.name, self.velocity)
    } */
}

impl Description for SpaceStation {
    fn describe(&self) -> String {
        format!("the {} flying  {} miles high, with {} crew members on board", self.name, self.altitude, self.crew_size)
    }
}

fn print_type<T: fmt::Debug>(item: T) {
    println!("{:?} is {:?}", item, any::type_name::<T>());
}

// fn compare_and_print<T: fmt::Display + PartialEq + From, U: fmt::Display + PartialEq + Copy>(a: T, b:U) {
fn compare_and_print<T, U>(a: T, b:U) 
    where   T: fmt::Display + PartialEq + From<U>, 
            U: fmt::Display + PartialEq + Copy
    {
    if a == T::from(b) {
        println!("{} is equal to {}", a, b);
    } else {
        println!("{} is not equal to {}", a, b);
    }
}

fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72
    };

    let gps = Satellite {
        name: String::from("GPS"),
        velocity: 2.42
    };

    let iss = SpaceStation {
        name: String::from("International Sapce Station"),
        crew_size: 6,
        altitude: 254
    };

    println!("hubble is {}", hubble.describe());
    println!("iss is {}", iss.describe());
    println!("hubble == gps is {}", hubble == gps);
    println!("hubble > gps is {}", hubble > gps);
    print_type(13);
    print_type(13.0);
    print_type("thirteen");
    print_type([12,13,56,78]);

    compare_and_print(1.0, 1);
    compare_and_print(1.1, 1);
}