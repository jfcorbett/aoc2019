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
    // let mut pos: Vec<Vec<i32>> = vec![vec![-1, 0, 2], vec![2, -10, -7], vec![4, -8, 8], vec![3, 5, -1]];
    for _ in 0..1000 {
        moondance(&mut pos, &mut vlc);
    }
    println!("{:?}", energy(&pos, &vlc));
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

fn energy(pos: &Vec<Vec<i32>>, vlc: &Vec<Vec<i32>>) -> i32 {
    (0..pos.len()).map(|m| 
        (0..3).map(|dim| pos[m][dim].abs()).sum::<i32>() * 
        (0..3).map(|dim| vlc[m][dim].abs()).sum::<i32>())
        .sum()
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

    // #[test]
    // fn test_energy() {
    //     unimplemented!();
    // }

}