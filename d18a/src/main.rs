use bimap::BiMap;
use std::collections::{HashSet, HashMap, VecDeque};

// use std::{
//     fs::File,
//     io::{Read, Result},
//     path::Path,
// };
// 
// fn read_whole_file(s: impl AsRef<Path>) -> Result<String> {
//     let mut file = File::open(s)?;
//     let mut s = String::new();
//     file.read_to_string(&mut s)?;
//     Ok(s)
// }

fn main() {
    // let zyrgwunk = read_whole_file(Path::new("input")).unwrap();
    // let input_sequence = parse_input_sequence("03036732577212944063491565474664");
//     let input = "#########
// #b.A.@.a#
// #########";

    // println!("{:?}", do_it(&input_sequence));
}

struct Maze {
    passages: HashSet<(usize, usize)>, 
    wpt_pos: BiMap<char, (usize, usize)>, 
    door_pos: BiMap<char, (usize, usize)>,
}

impl Maze {

    fn waypoint_pair_obstacles(&self) -> HashMap<(char, char), (usize, HashSet<char>)> {
        let mut wpo = HashMap::new();
        for (from_key, from_keypos) in &self.wpt_pos {
            for (to_key, obstacles) in self.waypoint_obstacles(*from_keypos) {
                let kp = wpt_pair(*from_key, to_key);
                if !wpo.contains_key(&kp) {
                    wpo.insert(kp, obstacles);
                }
            }

        }
        wpo
    }

    fn waypoint_obstacles(&self, from_pos: (usize, usize)) -> HashMap<char, (usize, HashSet<char>)> {
        let mut wpt_obst = HashMap::new();
        let mut visited = HashSet::new();
        let mut parent = HashMap::new();
        let mut distance = HashMap::new();
        let mut obstructing_doors = HashMap::new();
        let mut discovered_queue = VecDeque::new();

        // Initialize starting position and enqueue it
        distance.insert(from_pos, 0);
        obstructing_doors.insert(from_pos, HashSet::new());
        discovered_queue.push_back(from_pos);

        while !discovered_queue.is_empty() {
            // Dequeue, visit and analyze
            let cur_pos = discovered_queue.pop_front().unwrap();
            visited.insert(cur_pos);
            if self.wpt_pos.contains_right(&cur_pos) && cur_pos != from_pos {
                // It's a waypoint! Save distance and set of obstructing doors
                wpt_obst.insert(*self.wpt_pos.get_by_right(&cur_pos).unwrap(), 
                    (distance[&cur_pos], obstructing_doors[&cur_pos].clone()));
            } else if self.door_pos.contains_right(&cur_pos) {
                // It's a door... Add this door to cur_pos's outdated set of obstructing doors 
                obstructing_doors.get_mut(&cur_pos).unwrap()
                    .insert(*self.door_pos.get_by_right(&cur_pos).unwrap());
            }

            // Initialize and enqueue unvisited neighbours
            for adj_pos in self.adj_passages(cur_pos) {
                if !visited.contains(&adj_pos) {
                    parent.insert(adj_pos, cur_pos);
                    distance.insert(adj_pos, distance[&cur_pos] + 1);
                    obstructing_doors.insert(adj_pos, obstructing_doors[&cur_pos].clone());
                    discovered_queue.push_back(adj_pos);
                }
            }

            // Free brain space
            parent.remove(&cur_pos);
            distance.remove(&cur_pos);
            obstructing_doors.remove(&cur_pos);
        }
        
        wpt_obst
    }

    fn adj_passages(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        vec![
            (pos.0 - 1, pos.1),
            (pos.0 + 1, pos.1),
            (pos.0, pos.1 - 1),
            (pos.0, pos.1 + 1),
        ].iter().filter(|pos| self.passages.contains(pos)).map(|p| *p).collect::<Vec<_>>()
    }
}

fn wpt_pair(key1: char, key2: char) -> (char, char) {
    if key1 < key2 {(key1, key2)} else {(key2, key1)}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_waypoint_pair_obstacles() {
        let maze = parse_maze("#########
#b.A.@.a#
#########");
        let iko = maze.waypoint_pair_obstacles();
        assert_eq!(iko[&wpt_pair('@', 'a')], (2, HashSet::new()));
        assert_eq!(iko[&wpt_pair('@', 'b')], (4, vec!['a'].iter().map(|x| *x).collect::<HashSet<_>>()));
        assert_eq!(iko[&wpt_pair('a', 'b')], (6, vec!['a'].iter().map(|x| *x).collect::<HashSet<_>>()));
    }

    #[test]
    fn test_waypoint_obstacles_ex0() {
        let maze = parse_maze("#########
#b.A.@.a#
#########");
        let wo = maze.waypoint_obstacles((1,5));
        assert_eq!(wo[&'a'], (2, HashSet::new()));
        assert_eq!(wo[&'b'], (4, vec!['a'].iter().map(|x| *x).collect::<HashSet<_>>()));
    }

    #[test]
    fn test_waypoint_obstacles_ex2() {
        let maze = parse_maze("#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################");
        let wo = maze.waypoint_obstacles((4,8));
        assert_eq!(wo[&'a'], (3, HashSet::new()));
        assert_eq!(wo[&'p'], (10, vec!['h'].iter().map(|x| *x).collect::<HashSet<_>>()));
    }

    #[test]
    fn test_adj_passages() {
        let maze = parse_maze("#########
#b.A.@.a#
#########");
        let a = maze.adj_passages((1,2));
        assert!(a.contains(&(1,1)));
        assert!(a.contains(&(1,3)));
        assert!(!a.contains(&(0,2)));
        assert!(!a.contains(&(2,2)));
    }

    #[test]
    fn test_wpt_pair() {
        assert_eq!(('a', 'b'), wpt_pair('a', 'b'));
        assert_eq!(('a', 'b'), wpt_pair('b', 'a'));
        assert_eq!(('@', 'a'), wpt_pair('a', '@'));
        assert_eq!(('@', 'a'), wpt_pair('@', 'a'));
    }

    #[test]
    fn test_parse_maze() {
        let maze = parse_maze("#########
#b.A.@.a#
#########");
        assert_eq!(maze.wpt_pos.get_by_left(&'@'), Some(&(1,5)));

        assert_eq!(maze.wpt_pos.get_by_left(&'a'), Some(&(1,7)));
        assert_eq!(maze.wpt_pos.get_by_right(&(1,1)), Some(&'b'));

        assert_eq!(maze.door_pos.get_by_left(&'a'), Some(&(1,3)));

        assert!(maze.passages.contains(&(1,1))); // key
        assert!(maze.passages.contains(&(1,2))); // passage with nothing special
        assert!(maze.passages.contains(&(1,3))); // door
        assert!(maze.passages.contains(&(1,5))); // start pos
        
        assert!(!maze.passages.contains(&(1,0))); // wall
        assert!(!maze.passages.contains(&(1,8))); // wall
        assert!(!maze.passages.contains(&(42,666))); // void
    }
}

fn parse_maze(maze: &str) -> Maze { 
    let mut passage_pos = HashSet::new();
    let mut wpt_pos = BiMap::new();
    let mut door_pos = BiMap::new();

    for (i, line) in maze.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if ch.is_lowercase() || ch == '@' {
                wpt_pos.insert(ch, (i,j));
                passage_pos.insert((i,j));
            } else if ch.is_uppercase() {
                door_pos.insert(ch.to_lowercase().to_string().chars().next().unwrap(), (i,j));
                passage_pos.insert((i,j));
            } else if ch == '.' {
                passage_pos.insert((i,j));
            }
        }
    }
    return Maze{
        passages: passage_pos, 
        wpt_pos, 
        door_pos
    }
}

