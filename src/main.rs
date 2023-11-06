use crate::cars::parts::Axle;

pub mod cars;

fn main() {
    let car_part = Axle {width: 2.3};

    println!("Car axle is of width {:?}m", car_part.width)
}
