use std::iter::repeat;

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
    let zyrgwunk = read_whole_file(Path::new("input")).unwrap();
    let input:Vec<_> = zyrgwunk.trim().chars().map(|c| c.to_digit(10).unwrap()).collect();
    println!("{:?}", input);

}

fn do_phase() {
    unimplemented!();
}

fn weird_mult(v1: Vec<i32>, v2: Vec<i32>) -> i32 {
    v1.iter().zip(v2.iter()).map(|(n1,n2)| n1*n2).sum::<i32>().abs() % 10
}

fn wave(freq: usize, length: usize) -> Vec<i32> {
    // Make wave!
    vec![0,1,0,-1].iter().flat_map(|n| repeat(*n).take(freq)).cycle().skip(1).take(length).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weird_mult() {
        assert_eq!(weird_mult(vec![1,2,3,4,5,6,7,8], wave(1,8)), 4);
        assert_eq!(weird_mult(vec![1,2,3,4,5,6,7,8], wave(2,8)), 8);
        assert_eq!(weird_mult(vec![1,2,3,4,5,6,7,8], wave(3,8)), 2);
        assert_eq!(weird_mult(vec![1,2,3,4,5,6,7,8], wave(4,8)), 2);
        assert_eq!(weird_mult(vec![1,2,3,4,5,6,7,8], wave(5,8)), 6);
        assert_eq!(weird_mult(vec![1,2,3,4,5,6,7,8], wave(6,8)), 1);
        assert_eq!(weird_mult(vec![1,2,3,4,5,6,7,8], wave(7,8)), 5);
        assert_eq!(weird_mult(vec![1,2,3,4,5,6,7,8], wave(8,8)), 8);
    }
    
    #[test]
    fn test_pat() {
        assert_eq!(wave(1, 5), vec![1,0,-1,0,1]);
        assert_eq!(wave(2, 9), vec![0,1,1,0,0,-1,-1,0,0]);
        assert_eq!(wave(3, 15), vec![0,0,1,1,1,0,0,0,-1,-1,-1,0,0,0,1]);
    }
}

