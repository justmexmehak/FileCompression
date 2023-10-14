use std::io::{self, ErrorKind, Write};
use std::fs::{File, OpenOptions};

use crate::generate_random::generate_random_data;

pub fn get_file(path: &str) -> File {
    let file_open_res = File::open(path);

    let file = match file_open_res {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                create_file(path).expect("Problem creating the file");
                match File::open(path) {
                Ok(file) => file,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            }},
            other_error => panic!("Problem opening the file: {:?}", other_error),
        }
    };

    file
}

pub fn create_file(path: &str) -> Result<(), io::Error> {
    // let mut file = File::create(path)?;

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;
    
    let data = generate_random_data();
    write_to_file(&mut file, data)?;
    
    Ok(())
}

pub fn write_to_file(file: & mut File, data: String) -> Result<(), io::Error>{
    file.write_all(data.as_bytes())?;
    Ok(())
}
