#![allow(unused_variables)]
const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let (missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
    let jet = 2;

    println!("Firing {} of my {} missles...", ready, missiles);
    println!("{} missiles left", missiles - ready);
}
