extern crate num;

use num::PrimInt;

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
    let sgrofnak = read_whole_file(Path::new("input")).unwrap();
    let sgrofnak = sgrofnak
        .replace("<x=", "")
        .replace(" y=", "")
        .replace(" z=", "")
        .replace(">", "");
    let mut pos: Vec<Vec<i32>> = sgrofnak
        .lines()
        .map(|l| l.split(",").map(|n| n.parse::<i32>().unwrap()).collect())
        .collect();
    let mut vlc: Vec<Vec<i32>> = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
    // let mut pos: Vec<Vec<i32>> = vec![vec![0, 4, 0], vec![-10, -6, -14], vec![9, -16, -3], vec![6, -1, 2]];
    // let mut pos: Vec<Vec<i32>> = vec![vec![-1, 0, 2], vec![2, -10, -7], vec![4, -8, 8], vec![3, 5, -1]];
    // let mut pos: Vec<Vec<i32>> = vec![vec![-8, -10, 0], vec![5, 5, 10], vec![2, -7, 3], vec![9, -8, -3]];

    let pos_orig = pos.clone();
    let vlc_orig = vlc.clone();
    let mut rs:Vec<i32> = Vec::new();
    for dim in 0..3 {
        let mut n_steps = 0_i32;
        let mut pos = pos_orig.clone();
        let mut vlc = vlc_orig.clone();
        loop {
            n_steps += 1;
            moondance(&mut pos, &mut vlc);
            if n_steps == 1468 {
                println!("{:?}", pos);
            }
            if pos.iter().map(|p| p[dim]).collect::<Vec<_>>() == pos_orig.iter().map(|p| p[dim]).collect::<Vec<_>>() 
                && vlc.iter().map(|p| p[dim]).collect::<Vec<_>>() == vlc_orig.iter().map(|p| p[dim]).collect::<Vec<_>>() 
                {
                break
            }
        }
        rs.push(n_steps);
        println!("{:?}", n_steps);
        // println!("{:?}", pos);
        // println!("{:?}", vlc);
        // println!("{}: {:?}", n_steps, pos);
    }
    // Calculate Lowest Common Multiple of the three numbers of steps for x,y,z dims....
    // let n:num::PrimInt = num::PrimInt::from(rs[0]);
    // println!("{}",n.lcm(n));
    // println!("{}", lcm(rs[0] as num::Integer, &rs[1]));
}

fn moondance(pos: &mut Vec<Vec<i32>>, vlc: &mut Vec<Vec<i32>>) {
    // Calculate velocity
    for it in 0..vlc.len() {
        let pt = &pos[it];
        for ps in pos.iter() {
            for dim in 0..3 {
                vlc[it][dim] += (ps[dim]-pt[dim]).signum();
            }
        }
    }
    // Apply velocity
    for i in 0..vlc.len() {
        for dim in 0..3 {
            pos[i][dim] += vlc[i][dim];
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moondance() {
        let mut pos: Vec<Vec<i32>> = vec![vec![-1, 0, 2], vec![2, -10, -7], vec![4, -8, 8], vec![3, 5, -1]];
        let mut vlc: Vec<Vec<i32>> = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
        moondance(&mut pos, &mut vlc);
        assert_eq!(pos, vec![vec![2, -1, 1], vec![3, -7, -4], vec![1, -7, 5], vec![2, 2, 0]]);
        assert_eq!(vlc, vec![vec![3, -1, -1], vec![1, 3, 3], vec![-3, 1, -3], vec![-1, -3, 1]]);
    }


}