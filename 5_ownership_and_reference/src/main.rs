// Silence some warnings so they don't distract
#![allow(unused_mut, unused_variables)]

fn main() {
    // This either gets the first argument as a String, or prints
    // usage and exits if an argument was not supplied to the program.
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });

    inspect(&arg);

    fn inspect(s: &String) {
        println!(
            "{}",
            if s.ends_with("s") {
                "plural"
            } else {
                "singular"
            }
        );
    }

    change(&mut arg);
    println!("I have many {}", arg);

    fn change(a: &mut String) {
        // function takes a mutable reference to a string
        if !a.ends_with("s") {
            a.push_str("s") // adds "s" to the string
        };
    }

    if eat(arg) {
        println!("Might be bananas");
    } else {
        println!("Not bananas");
    }

    fn eat(bol: String) -> bool {
        bol.starts_with("b") && bol.contains("a") // a function accepting ownership of the string
    }

    let mut material = "mud".to_string();
    println!("This material is just `{}`.", material);
    bedazzle(&mut material);
    println!("Wow! Now the material is `{}`!", material);

    fn bedazzle(arg: &mut String) {
        *arg = "sparkly".to_string(); //using dereferene of mutable reference to assign it a new value
    }
}
