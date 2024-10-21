#[allow(unused_variables)]
#[warn(unused_assignments)]
#[warn(unused_imports)]

/// Main documentaiton as this helps when building the docs with
/// cargo docs
use std::io;

const URL: &str = "google.com";
fn main() {

    //!```
    //! fn main() {...}
    //! ```

    // let mut input = String::new();
    // println!("Say Something!");
    // match io::stdin().read_line(&mut input) {
    //     Ok(_) => {
    //         println!("You said {}", input);
    //     },
    //     Err(e) => {
    //         println!("Something happend! {}", e);
    //     }
    // }

    // by default, numbers are set to int32

    println!("My name is {1}, I'm a {0} and i like rice and {2}", "samuel", "boy", "beans");
    println!("{name} {surname} is a {jd}", name="sam", surname="tai", jd="mle");
    println!("binary: {:b}, hex: {:x}, octal:{:o}", 5,5,5 );
    println!("Array: {:?} ", [1, 2, 3]);
    let name = "Bayo";
    let mut surname = "Dammy"; //mutable variable
    let afe = 43;
    
    let (a, b, c) = (2, true ,"grey");
    let million: i32 = 1_000_000;
    let def: i64 = 98765432109;
    surname = "bamgboshe";
    println!("{1}, {2}, {0}, {3}", name, afe, def, surname);

    let char1 = 'r';
    println!("{char1}");

    // string slices
    // string slices are immutable

    let cat: &str = "Fluffy";
    let cat: &'static str = "Flufer"; // defining the life of teh variable

    println!("{cat}");

    // string objects
    // this creates an empty list
    let dog = String::new();
    let mut dog = String::from("Max");
    println!("{dog}");

    let owner = format!("Hi am {} the owner of {}", surname, dog); // format macro
    println!("{}", owner);

    // operations on string
    println!("{}", dog.len());
    dog.push(' '); // for this to work, dog has to mutable
    dog.push_str("the dog!");
    println!("{}", dog);

    let new_dog = dog.replace("the", "my dog");
    println!("{}", new_dog);

    /*
    Other string methods includes
    - split
    - split_whitespace
    - chars
    - etc
    */

    println!("{}", URL);

    let nomar: &str = "samu";
    let new_nomar: &str = &nomar.replace("samu", "damudoe");
    println!("{}", new_nomar);

    for i in 1..6 { // for i in range 1 - 6
        println!("{}", i);
        say_hi();
        }

    let namer = "Doe"     ;
    say_name(namer)
}

fn say_hi() {
    
    println!("Hello there!");
}

fn say_name(name: &str) {
    println!("Hello {},", {name})
}