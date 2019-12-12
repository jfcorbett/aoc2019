use std::{
    fs::File,
    io::{Read, Result},
    path::Path,
};

fn read_whole_file(s: impl AsRef<Path>) -> Result<String> {
    let mut file = File::open(s)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    let whole_file = read_whole_file(Path::new("input")).unwrap(); // String
    // let whole_file = String::from("0222112222120000");
    let nums: Vec<_> = whole_file
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let width: usize = 25;
    let height: usize = 6;
    let n_pixels = width * height;
    let n_layers = nums.len() / n_pixels;

    // let xx = (0..n_layers)
    //     .map(|l| nums[l*n_pixels..(l+1)*n_pixels].iter().filter(|&n| *n == 0_u8).count());
    // let xx = (0..n_pixels).map(|p| (p..nums.len()).step_by(n_pixels).collect::<Vec<_>>());
    // let yy = (0..n_pixels).map(|p| nums.chunks(n_pixels).map(move |l| l[p]).collect::<Vec<_>>().iter().position(|&n| n==0));
    let pixel_layers = (0..n_pixels)
        .map(|p| nums.chunks(n_pixels).map(move |l| l[p]).collect::<Vec<_>>()).collect::<Vec<_>>();

    let pxpos0 = pixel_layers.iter().map(|p| p.iter().position(|&n| n==0)).collect::<Vec<_>>();
    let pxpos1 = pixel_layers.iter().map(|p| p.iter().position(|&n| n==1)).collect::<Vec<_>>();

    // let pixel_colors = pixel_layers.iter().map(|p| 
    //         match p.iter().position(|&n| n==0) < p.iter().position(|&n| n==1) {
    //             true => 0, 
    //             false => 1,
    //         } 
    //     ).collect::<Vec<_>>();
    // let color_depths = pixel_layers.iter().map(|p| (0..=2).map(|c| p.iter().position(|&n| n==0))).collect::<Vec<_>>();
    let pixel_colors = pixel_layers.iter().map(|p| 
            match p.iter().position(|&n| n==0) {
                None => 1,
                Some(d0) => match p.iter().position(|&n| n==1) {
                    None => 0,
                    Some(d1) => match d0 < d1 {
                        true => 0, 
                        false => 1,
                    } 
                }
    
            }

        ).collect::<Vec<_>>();

    let img = pixel_colors.chunks(width).map(|row| row.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("")).collect::<Vec<_>>().join("\n");
        // .map(|px| px.iter().position(|n| n==0));
    println!("{}", &img.replace("0"," ").replace("1", "@"));


    // println!("{:?}", pixel_colors);
    // println!("{:?}", pixel_layers);
    // println!("{:?}", pxpos0);
    // println!("{:?}", pxpos1);
}
