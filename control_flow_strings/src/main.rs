
#![allow(dead_code, unused_mut, unused_variables)]

fn main() {
    // This collects any command-line arguments into a vector of Strings.
    // For example:
    //
    //     cargo run apple banana
    //
    // ...produces the equivalent of
    //
    //     vec!["apple".to_string(), "banana".to_string()]
    let args: Vec<String> = std::env::args().skip(1).collect();

    // This consumes the `args` vector to iterate through each String
    for arg in args {

        if arg == "sum" {
            sum();
        }else if arg == "double" {
            double();
        } else {
            count(arg);
        }

    }
}

fn sum() {
    let mut sum = 0;

    for i in 7..=23 {
        sum += i;
    }

    println!("The sum is {}", sum);
}

fn double() {
    let mut count = 0;
    let mut x = 1;

    while x< 500 {
        count +=1;
        x *= 2; 
    }

    println!("You can double x {} times until x is larger than 500", count);
}

fn count(arg: String) {

    let mut i = 0;
    loop {
        println!("{}", arg);
        i += 1;
        if i == 8 {
            break;
        }
    }

    println!(); // This will output just a newline at the end for cleanliness.
}
