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
            for m in movs {
                let (dir, n) = m.split_at(1);
                let n = n.parse::<i32>().unwrap();
                // TODO cabpos.insert((x,y));
                cabpos.insert((dir.len(), n));
            }
            pos_by_cable.push(cabpos);
        }
    }
    println!("{:?}", pos_by_cable);
    // Find intersections
    // Calculate Manhattan distances
    // Pick closest
}

//fn steps(start_pos: &Tuple, movement: &String) {}
