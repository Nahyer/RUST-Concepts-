// Silences some warnings
#![allow(dead_code, unused_mut, unused_variables)]

// the program run after adding arguments to the "cargo run" e.g "cargo run sum"
fn main() {
    // This collects any command-line arguments into a vector of Strings.
    // vec!["apple".to_string(), "banana".to_string()]

    let args: Vec<String> = std::env::args().skip(1).collect();

    // This consumes the `args` vector to iterate through each String
    for arg in args {
        if arg == "sum" {
            sum();
        } else if arg == "double" {
            double();
        } else {
            count(arg);
        }
    }
}

fn sum() {
    let mut sum = 0;
    for mut num in 7..=23 {
        //for loop iterates through range 7-23 while adding the numbers
        sum += num;
    }

    println!("The sum is {}", sum);
}

fn double() {
    let mut count = 0;
    let mut x = 1;

    while x <= 500 {
        // a while loop that multiplies x by 2 to 500
        x *= 2;
        count += 1;
    }

    println!(
        "You can double x {} times until x is larger than 500",
        count
    );
}

fn count(arg: String) {
    let mut counter = 0;
    
    loop {
        // an unconditinal statement that prints arg 8 times then breaks.
        print!("{} ", arg);
        counter += 1;
        println!("{} ", counter);
        if counter == 8 {
            break;
        }
    }

    println!(); // This will output just a newline at the end for cleanliness.
}
