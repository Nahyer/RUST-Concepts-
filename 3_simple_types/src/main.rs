#![allow(dead_code, unused_variables)]

use ding_machine::{ding, on_off, print_array, print_difference, print_distance};
fn main() {
    let coords: (f32, f32) = (6.3, 15.0);
    print_difference(coords.0, coords.1); // tuple indexing practice.

    let coords_arr: [f32; 2] = [coords.0, coords.1]; // creating an array literal out of parts of `coord`
    print_array(coords_arr); // and passing it in this  function.

    let series = [1, 1, 2, 3, 5, 8, 13];
    ding(series[6]); // using array indexing to pass a value to a function.

    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");
    on_off(mess.2[1].0); //passing a value with both tuple and array indexing

    print_distance(coords);
}
