use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

fn main() {
    let input_file = String::from("input");
    let mut file_contents = String::new();
    let mut f = File::open(input_file).expect("Unable to open file");
    f.read_to_string(&mut file_contents)
        .expect("Unable to read string");
    let cables = file_contents.split("\n");

    let mut pos_by_cable = Vec::new();
    for c in cables {
        if c.len() > 0 {
            let mut cabpos = HashSet::new();
            let movs = c.split(",");
            let mut x = 0;
            let mut y = 0;
            for m in movs {
                let (dir, n) = m.split_at(1);
                let n = n.parse::<i32>().unwrap();
                let (dx, dy) = incr(dir);
                for _ in 0..n {
                    x += dx;
                    y += dy;
                    cabpos.insert((x,y));
                }
            }
            pos_by_cable.push(cabpos);
        }
    }
    // println!("{:?}", pos_by_cable);
    // Find intersections
    let c1: &HashSet<(i32,i32)> = &pos_by_cable[0];
    let c2: &HashSet<(i32,i32)> = &pos_by_cable[1];
    let crossings = c1.intersection(c2);

    // Calculate Manhattan distances
    let mut mds: Vec<i32> = Vec::new();
    for c in crossings {
        mds.push((c.0).abs() + (c.1).abs());
    }
    // Pick closest
    mds.sort(); 
    println!("{:?}", mds[0]);
}

fn incr(dir: &str) -> (i32, i32) {
    match dir {
        "R" => (1,0),
        "L" => (-1,0),
        "U" => (0,1),
        "D" => (0,-1),
        _ => panic!("WTF is {}", dir),
    }
}
//fn steps(start_pos: &Tuple, movement: &String) {}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_incr() {
        assert_eq!(incr("R"), (1,0));
    }
}