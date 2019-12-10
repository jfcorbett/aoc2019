use std::{
    fs::File,
    io::{Read, Result},
    path::Path,
};

// The image you received is 25 pixels wide and 6 pixels tall.

// To make sure the image wasn't corrupted during transmission, 
// the Elves would like you to find the layer that contains the fewest 0 digits. 
// On that layer, what is the number of 1 digits multiplied by the number 
// of 2 digits?

fn read_whole_file(s: impl AsRef<Path>) -> Result<String> {
    let mut file = File::open(s)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    println!("Hello, world!");
}
