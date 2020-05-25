/*
Authors: Awildo G., Sosina A., Siqi F., Sam A.
Project: Advent of Code 2019 Day 2 Part 1
Date: 05/25/2020
Desc.: We accept input four integers at a time, with the first being an operation,
*  the next two being operands, and the last being an index to place the result on. 
*/

use std::io; // for reading file name
use std::fs; // for reading file

fn main() {
    println!("Enter filename:");
    
    // read the file name
    let mut file_name = String::new();
    io::stdin()
        .read_line(&mut file_name)
        .expect("Invalid input");

    let file_name = file_name.trim();

    // read the file
    // read_to_string outputs an io::Result (Ok or Err)
    // contents is the whole file in a &str
    let contents = fs::read_to_string(file_name).expect("File didn't open"); 
    
    // split it for each ',' and put each item into a vector
    // collect() puts each item that was split into a vector
    let contents: Vec<&str> = contents.split(",").collect(); 

    // We will parse each string in contents into an integer and put it into codes.
    let mut codes: Vec<u32> = Vec::new();

    for code in contents.iter() {
        codes.push(code.trim().to_string().parse().expect("Not a number"));
    }
    
    // // print for debugging
    // for line in contents.iter() {
    //     print!("{} ", line);
    // }

    // go through contents, each fourth item starting at index 0 is an
    // instruction
    // 1 means to add
    // 2 means to multiply
    // 99 means to halt
    let mut index = 0;
    while index < (codes.len() - 4) {
        let operation: u32 = codes[index];

        // e.g. op 2 3 4
        // The second and third item are indices in the contents vector for
        // actual items we will operate on
        // So, we do some operation on contents[2] and contents[3] and place it at contents[4]
        
        // because vectors can only be indexed by usize types
        let item1 = codes[codes[index + 1] as usize];
        let item2 = codes[codes[index + 2] as usize];
        let resulting_loc = codes[index + 3];

        // println!("Item 1 is {} and Item 2 is {}", item1_loc, item2_loc); // for debugging

        // adding
        if operation == 1 {
            codes[resulting_loc as usize] = item1 + item2;
        }
        // multiplying
        else if operation == 2 {
            codes[resulting_loc as usize] = item1 * item2;
        }
        // halt
        else if operation == 99 {
            break;
        } 
        else {
            println!("Invalid input!");
            break;
        }
        
        index += 4;
    }

    println!("Position 0: {}", codes[0]);
}

