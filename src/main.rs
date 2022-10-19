
use std::env::args;
use std::fs::{File};
use std::process::exit;
use std::io::{BufReader, BufRead};

fn main() {
    let args:Vec<String> = args().collect();
    if args.len() < 2 {
        println!("\nNeed path to inputs file.\n");
        exit(1);
    }

    let path = &args[1];

    //formatting strings
    //One way:
    //let err_msg = format!("\nFile {} was not found.\n", path);
    //Another way (if possible):
    let err_msg = format!("\nFile {path} was not found.\n");

    let file_result = File::open(path);
    let file;
    match file_result {
        Ok(f) => { file = f; },
        _ => {
            println!("{}", err_msg);
            exit(1);
        }
    }

    //READ FILE METHOD 1 (all at once):
    /*
    let contents_as_string = fs::read_to_string(path).unwrap();
    println!("{contents_as_string}");
    */

    //READ FILE METHOD 2 (all at once):
    /*
    let contents_as_vector = fs::read(path).unwrap();
    println!("{:?}", contents_as_vector);
    */

    //READ FILE METHOD 3 (all at once):
    /*
    let mut contents_as_buffer = vec![0; 15000];
    let count = file.read(&mut contents_as_buffer).unwrap();
    println!("{:?}", contents_as_buffer);
    println!("\n{count}\n");
    */

    //READ FILE METHOD 4 (line-by-line):
    let reader = BufReader::new(file);
    for (index, line) in reader.lines().enumerate() {
        let binary_string = line.unwrap();
        println!("{index}, {binary_string}");

        let bits_chars:Vec<char> = binary_string.chars().collect();
        println!("{:?}", bits_chars);


        //FOR LOOP - VERSION 1
        /* 
        for bit_idx in 0..bits_chars.len() {
            if bits_chars[bit_idx] == '1' {
                println!("\tPosition {bit_idx} is a 1.");
            }
        }
        println!("");
        */ 

        //FOR LOOP - VERSION 2
        for (idx, bit) in bits_chars.iter().enumerate() {
            if *bit == '1' {
                println!("\t\tPosition {idx} is a {bit}.");
            }
        }
        println!("");



    }
}
