mod PIAIC {
     pub mod IOT {
         pub fn quarter() {
            println!("QUARTER 1");
        }
    }
}


use crate::PIAIC::IOT;
fn main() {
    IOT::quarter();
}