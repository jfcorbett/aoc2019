use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{Read, Result},
    path::Path,
};

#[derive(Debug)]
struct Move {
    dir: String,
    num_steps: i32,
}

fn read_whole_file(s: impl AsRef<Path>) -> Result<String> {
    let mut file = File::open(s)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

fn parse_move(m: &str) -> Move {
    let (dir, n) = m.split_at(1);
    Move {
        dir: String::from(dir),
        num_steps: n.parse::<i32>().unwrap(),
    }
}

fn parse_all_moves(all_moves: &str) -> Vec<Vec<Move>> {
    let mut moves = Vec::new();
    for line in all_moves.lines() {
        let m = line
            .split(",")
            .map(|mov| parse_move(mov))
            .collect::<Vec<_>>();
        moves.push(m);
    }
    moves
}

fn trace_step_dists(moves: &Vec<Move>) -> HashMap<(i32, i32), i32> {
    let mut steps: HashMap<(i32, i32), i32> = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    let mut dist = 0;
    for m in moves {
        let (dx, dy) = incr(&m.dir);
        for _ in 0..m.num_steps {
            x += dx;
            y += dy;
            dist += 1;
            steps.insert((x, y), dist);  // BUG if cable may cross itself...
        }
    }
    steps
}

fn incr(dir: &str) -> (i32, i32) {
    match dir {
        "R" => (1, 0),
        "L" => (-1, 0),
        "U" => (0, 1),
        "D" => (0, -1),
        _ => panic!("WTF is {}", dir),
    }
}

fn main() {
    let whole_file = read_whole_file(Path::new("input")).unwrap(); // String
    // let moves = parse_all_moves(&whole_file);
    
    // Get distance to each step along the cables
    let step_dists = parse_all_moves(&whole_file).iter().map(|movs| trace_step_dists(&movs)).collect::<Vec<_>>();
    // Now just keep the steps to find the intersections (crossings)
    let steps = step_dists.iter().map(|stps| stps.keys().collect::<HashSet<_>>()).collect::<Vec<_>>();
    // let crossings = steps[0].intersection(&steps[1]).collect::<HashSet<_>>();
    // Get intersections and look up total distance to each
    let min_dist = &steps[0].intersection(&steps[1]).map(|&c| step_dists[0].get(c).unwrap() + step_dists[1].get(c).unwrap()).min();

    println!("{:?}", min_dist);
}
