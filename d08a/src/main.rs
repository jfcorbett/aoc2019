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
    let whole_file = read_whole_file(Path::new("input")).unwrap(); // String
    let nums:Vec<_> = whole_file.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let width: usize = 25;
    let height: usize = 6;
    let n_pixels = width * height;
    let n_layers = nums.len() / n_pixels;

    // let xx = (0..n_layers)
    //     .map(|l| nums[l*n_pixels..(l+1)*n_pixels].iter().filter(|&n| *n == 0_u8).count());
    let xx = (0..n_layers)
        .map(|l| &nums[l*n_pixels..(l+1)*n_pixels])
        .map(|l| (0..=2).map(|d| l.iter().filter(|&n| *n == d).count()).collect::<Vec<_>>())
        .min_by(|a,b| a[0].cmp(&b[0])).unwrap();
    
    
    // for l in 0..n_layers {
    //     let layer = &nums[l*n_pixels..(l+1)*n_pixels];

    // }


    println!("{:?}", xx[1]*xx[2]);
}
