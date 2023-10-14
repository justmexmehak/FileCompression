use std::fs::{self, File};
use std::os::windows::prelude::*;
use std::io::{self, Read};

mod compress;
use crate::compress::compress_data;

mod generate_random;

mod prepare_file;
use crate::prepare_file::*;


fn main() {
    let og_file_path = "og_file.txt";
    let mut file = get_file(og_file_path);

    println!("Got OG file successfully");
    
    let data = match get_file_data(&mut file) {
        Ok(data) => data,
        Err(e) => panic!("Could not read file: {:?}", e)
    };


    let compressed_data = compress_data(data);

    let compressed_file_path = "compressed_file.txt";
    let compressed_file = match get_compressed_file(compressed_file_path, compressed_data) {
        Ok(file) => file,
        Err(e) => panic!("Could not compress file successfully: {:?}", e)
    };

    let og_file_size = match get_file_size(og_file_path) {
        Ok(size) => size,
        Err(error) => panic!("Cannot get file size {:?}", error),
    };

    let compressed_file_size = match get_file_size(compressed_file_path) {
        Ok(size) => size,
        Err(error) => panic!("Cannot get file size {:?}", error),
    };

    println!("Compressed File by {:.2}%", (og_file_size - compressed_file_size) as f64 / og_file_size as f64 * 100.0);
    println!("Original File Size: {}KB", og_file_size / 1024);
    println!("Compressed File Size: {}KB", compressed_file_size / 1024)
    

}



fn get_compressed_file(path: &str, data: String) -> Result<File, io::Error> {
    let mut compressed_file = File::create(path)?;
    write_to_file(& mut compressed_file, data)?;
    Ok(compressed_file)
}

fn get_file_size(path: &str) -> Result<u64, io::Error> {
    let meta_data = fs::metadata(path)?;
    let file_size = meta_data.file_size();
    Ok(file_size)
}


fn get_file_data(file: & mut File) -> Result<String, io::Error> {
    let mut data = String::new();
    file.read_to_string(& mut data)?;
    Ok(data)
}