use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::create("output.txt")
    .expect("Could not create file!");

    file.write_all(b"This file is create by Endy!")
    .expect("Cannot write to the file, sorry!");

    drop(file);
    file = File::open("output.txt")
    .expect("Could not open file! ");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
    .expect("Oops Can not read the file!");

    println!("Contents: \n\n{}", contents);

    drop(file);
}
