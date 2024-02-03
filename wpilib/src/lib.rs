use autocxx::prelude::*;

include_cpp! {
    #include "include.h"
    safety!(unsafe)
    generate!("frc::TimedRobot")
}

fn main() {
    println!("Hello, world!");
}
