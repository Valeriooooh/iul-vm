use std::{
    fs::File,
    io::{BufReader, Read},
};

fn main() {
    let buffer = read_file();
    for i in buffer.iter() {
        print!("{}", i.to_owned() as char);
    }
    println!("\nEOF");
    // cafe babe
    if !(validate_file(&buffer)) {
        panic!("Not .class file")
    }
    println!("{:?}", (&buffer[0..4]));
}

fn read_file() -> Vec<u8> {
    let f = match File::open("main.class") {
        Ok(a) => a,
        Err(e) => panic!("{e}"),
    };
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();
    let _ = reader.read_to_end(&mut buffer);
    buffer
}
fn validate_file(buf: &Vec<u8>) -> bool {
    let magic = vec![202, 254, 186, 190];
    let abuf = buf[0..4].to_owned();
    for i in 0..4 {
        if magic[i] != abuf[i] {
            return false;
        }
    }
    true
}
