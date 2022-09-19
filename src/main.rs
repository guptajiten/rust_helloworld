use rand::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, Read};
use std::collections::{VecDeque};

#[allow(unused)]
#[allow(irrefutable_let_patterns)] // without this a warning will be flagged by compiler for irrefutable if let used in code
fn main() {
    // Hello
    println!("Good day!, Hello world!"); // '!' used to call the macro println


    //General
    let _warning_variable = 2; //example if _ used then compiler doesnt show warning for unused var
    // no null datatype in rust
    // Some or None are used for data validation


    // Variables
    let mut city_pincode = 411012; // mutable integer by default it is i32, by default variables are immutable
    city_pincode += 3; // mutable hence modifiable
    let latitude: f32 = 18.5204; // float fixed, force the variable to f64 (Default for float)
    println!("Im in pune which has pincode: {}, latitude: {}", city_pincode, latitude);
    const EARTH_RADIUS_IN_KILOMETERS: f64 = 6371.0; // const can not be modified through out the program and need explicit type while declaring
    let gfg = Some(10); // using Some in declaration
    let mut pp: Option<(&str, f64, f64)> = None;


    // Strings
    let city_slice = "Pune"; // snake case variables
    let state_slice = "Maharashtra";
    let country_string = String::from("India");
    let city_state_concat1 = [city_slice, state_slice].concat(); // 1st way to concatinate
    println!("City State concatenation example1: {}", city_state_concat1);
    let city_state_concat1 = format!("{} {}", city_slice, state_slice); // another way to concatinate from macro format - which doesnt write the value to console like println, and other is concat!()
    println!("City State concatenation example2: {}", city_state_concat1);
    let mut another_city_string: String = "Bangalore".to_string();
    another_city_string.push_str("Karnataka"); // another way to concatinate
    println!("City State concatenation example3: {}", another_city_string);
    let mut slogan = String::new(); // even a new string is not mutable by default
    slogan.push_str("We hit the ground");
    slogan.push(' '); // remember the type of single quote used for char
    slogan = slogan + "every time";
    println!("Slogan: {}", slogan);


    // Casting
    let float_thirty_two: f32 = 17.2;
    let unsigned_eight: u8 = 5;
    //let result = float_thirty_two / unsigned_eight; // wrong - rust doesnt do implicit conversion
    let casr_unsigned_eight = unsigned_eight as f32;
    let result = float_thirty_two / casr_unsigned_eight; // correct 
    println!("Result: {}", result);
    let number: u8 = 65; // Cast a number as ascii. If used as float, u128 then compiler would throw error
    let cast_letter: char = number as char;
    println!("Cast number: {}", cast_letter);
    

    //Operator
    let squared = i32::pow(8,2);
    let float_integer = f32::powi(6.5, 3);
    let float_float = f32::powf(6.5, 3.14);
    println!("Squared: integer to integer {} float to integer {} float to float {}", squared, float_integer, float_float);
    let are_equal_is_true = 1 == 1;
    let are_equal_is_false = 1 == 2;
    let are_not_equal = 1 != 2;
    let is_true = true;
    // other operator &, | , ^, <<, >>


    // Control Flow
    let word = "Duck";
    if word == "Duck" {
        println!("Quack");
    } else if word == "Dog" || word == "BullDog" { // rust evaluates && first and then ||, use parenthesis to set the order of evaluation
        println!("Bark");
    } else {
        println!("All quet here");
    }
    if let word = "Dog" { // 'if let' example to replace match, here it is irrefutable because word will be Dog always. If compiler doesnt know what value it will be then it is refutable field
        println!("Bark again!");
    }
    enum NavigationAids {
        NDB,
        VOR,
        VORDME
    }
    println!("Enum NDB: {}", NavigationAids::NDB as u8);
    enum AnotherNavigationAids {
        NDB(u16),
        VOR(String, f32),
        VORDME(String, f32),
        FIX {name: String, latitude: f32, longitude: f32}
    }
    let fix_tarry = AnotherNavigationAids::FIX {
        name: String::from("TARRY"),
        latitude: 40.565,
        longitude: -83.8565
    };

    // Options and Match
    let phrase = String:: from("Duck AIrlines");
    let letter = phrase.chars().nth(5);
    //println!("nth char is {}", letter);
    let value = match letter { // match is similar to switch in another languages
        Some (character) => character.to_string(),
        None => String::from("None value")
    };
    println!("value from match {}", value);
    let animal = "Duck";
    match animal {
        "Duck" => println!("Quack from match"),
        "Dog" => println!("Bark from match"),
        _ => println!("All quet here")
    };
    let ndb_freq:u16 = 384;
    match ndb_freq {
        200..=500 => println!("NDB Frequncy is valid"),
        ndb_freq if ndb_freq > 500 && ndb_freq <= 600 => print!("NDB Frequncy is high"),
        _ => print!("NDB Frequency is invalid")
    };

    // another example of option and match
    let route = [
        ("KCLE", 41.0978, -98.00),
        ("FOD", 87.098, 90.8888)
    ];
    let mut total_distance = 0.0;
    let mut previous_waypoint: Option<(&str, f64, f64)> = None;
    for waypoint in route.iter() {
        match previous_waypoint {
            None => {
                previous_waypoint = Option::from((waypoint.clone()));
                continue;
            }
            Some(previous_waypoint_value) => {
                let previous_waypoint_radians = previous_waypoint_value.1.to_radians();
                let waypoint_radians = waypoint.1.to_radians();
            }
        }
    }


    // Loops - For loop, While loop, Loop
    loop { print!("hello again\n"); break;}
    let mut counter = 0;
    while counter <= 10 {
        counter += 1;
    };
    // while let mut counter = 10 {
    //     counter += 1;
    //     println!("{}", counter);
    //     if counter == 11 {
    //         break;
    //     }
    // };
    for index in 1..10{
        let value = 0;
    }
    let duck_aircraft = ["Boeing 737", "Boeing 767", "Boeng 787", "Airbus 319", "Airbus 320"];
    for aircraft in duck_aircraft.iter() {
        print!("{}",aircraft);
    }

    // Memory Management
    let original = String::from("I am here");
    let next = original; // original passed its ownership to next, ownership of next returns to original when next goes out of scope
    let next_borrow = &next; // borrowing example
    let mut original = String::from("original value");
    println!("\nouter original value: \t\"{}\"", original);
    {
        let next = &mut original; // next is pointer storing the address in stack and value in heap
        *next =  String::from("next value"); // dereference with *, to change the next we need to use mut, &mut
        println!("\ninner scope next: \t\"{}\"", next);
        println!("\ninner scope original: \t\"{}\"", original);
    }
    println!("\nouter scope original: \t\"{}\"", original);


    // closure example
    let write_message = |slogan: String| {
        println!("Hey, This is the closure. {}, {}", original, slogan);
    }; // note that closure ends with a semocolon

    // error handling
    //panic!("I am panicked");
    
    // file handling
    let filename = "/code/test-projects/rust_helloworld/Inavlid_README.md";
    match File::open(filename) {
        Ok(file) =>{
            println!("{:#?}", file);
        }
        Err(error) => {
            println!("{:#?}", error); // print error object
        }
        _ => {
            println!("default");
        }
    }
    
    // Data Structures - Struct, Traits
    struct Waypoint { // Pascal case
        name: String,
        latitude: f64,
        longitude: f64
    }
    let mut kcle = Waypoint { /// mut is all or nothing
        name: "KCLE".to_string(),
        latitude: 67.989,
        longitude: 87.09
    };
    let kslc = Waypoint {  // reuse the last var using dots
        name: "KSLC".to_string(),
        ..kcle
    };
    struct Segment {
        start: Waypoint,
        end: Waypoint
    }
    impl Segment {
        fn new(start: Waypoint, end: Waypoint) -> Self {
            Self {
                start,
                end
            }
        }

        fn distance(&self) -> f32{
            let distance = 324.019;
            distance as f32
        }
    }
    let kscle_kslc = Segment::new(kcle, kslc); // :: used in new
    let distance = kscle_kslc.distance(); // . used with custom functions, associated methods
    println!("Distance: {:.1}", distance); // example how to print only one decimal

    struct Boeing {
        required_crew: u8,
        range: u16
    }
    struct Airbus {
        required_crew: u8,
        range: u16
    }    
    trait Flight {
        fn is_legal(&self, required_crew: u8, available_crew: u8, range: u16, distance: u16) -> bool;
    }
    impl Flight for Boeing { // here we have actual implementation
        fn is_legal(&self, required_crew: u8, available_crew: u8, range: u16, distance: u16) -> bool {
            if (available_crew >= required_crew) && (range + 150 > distance) {
                true
            } else {
                false
            }
        }   
    }
    impl Flight for Airbus { // here we have actual implementation
        fn is_legal(&self, required_crew: u8, available_crew: u8, range: u16, distance: u16) -> bool {
            if (available_crew >= required_crew) && (range + 280 > distance) {
                true
            } else {
                false
            }
        }   
    }
    let boeing = Boeing {
        required_crew: 4,
        range: 7370
    };
    let airbus = Airbus {
        required_crew: 7,
        range: 5280
    };
    let boeing_is_legal = boeing.is_legal(boeing.required_crew, 18, boeing.range, 2385);
    let airbus_is_legal = airbus.is_legal(airbus.required_crew, 3, airbus.range, 2200);
    println!("Is the Boeing flight legal? {}\nIs the Airbus flieght legal? {}", boeing_is_legal, airbus_is_legal);

    //Collections - Sequences, Maps, Sets
    // Vec; works pretty well!
    let vec_macro = vec![1, 2, 3, 4, 5];
    let mut flights:Vec<&str> = Vec::new(); // push pop iter remove insert
    flights.push("DA113\t to Boston departs at 06:20");
    flights.push("BS113\t to Boston departs at 08:20");
    for flight in flights.iter() {
        println!("Flight details: {}", flight);
    }
    let second = flights[1]; // fast but risky, crash a program
    let first = flights.get(0); // safer but slower, wont crash a program
    match first {
        Some(flight) => {
            println!("{}", flight)
        }
        _=> {
            println!("default")       
        }
    }
    if let Some(flight_value) = flights.get(1) { // if we really care about getting the data back then just use 'if let'
        println!("flight value from \"if let\"\t{}", flight_value);
    }
    flights.remove(1);

    // Vectors Double Ended Queue
    // push_front, push_back, contains, iter(), iter_mut(), contains_key

    // HashMap; does not work well :(
    let mut map = HashMap::<String, String>::new();
    map.insert("some_key".to_string(), "some_value".to_string());
    map.insert("some_other_key".to_string(), "some_other_value".to_string());

    // Concurrency
    

    // Stepping in the random crate
    // Example for using Crate
    let x: u8 = random(); // explicit
    let y = random::<f64>();


    // Goodbye
    println!("Goodbye! Have a nice day!");
}

// function example with explicit lifetime defined ('a) - refer when you see memory management examples
// this is required as the scope of variable may go out of scope, p3 having pass by value hence no lifetime defined
// if 'a not used then compiler doesnt know which lifetime to used
// lifetime applies to reference only
fn explicit_lifetime<'a>(p1: &'a i32, p2: &'a i32, p3: i32) -> &'a i32 {
    if p1 > p2 {
        p1        // when return from function do not use ';'
    } else if p2 > &p3 {
        p2
    } else {
        p1
    }
} 

fn read_file(filename: &str) -> Result<String, Error> {
    let mut file_handle = File::open(filename)?; // note the ? here, if it fails then the function returns from here it self, need use std::io::Error;
    let mut file_data = String::new();
    file_handle.read_to_string(&mut file_data)?;
    Ok(file_data)
}

