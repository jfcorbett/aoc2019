use std::{
    collections::{HashMap, HashSet},
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
    let m = make_orb_map(&whole_file);
    println!("{:?}", m);
}

fn count_orbits_around(planet: &str, depth: u16, orbmap: &HashMap<&str, HashSet<&str>>) -> u16 {
    orbmap[&planet].len() as u16
}

fn make_orb_map(orbfile: &str) -> HashMap<&str, HashSet<&str>> {
    let mut orbmap: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in orbfile.lines() {
        let o:Vec<_> = line.split(")").collect();
        let children = orbmap.entry(o[0]).or_insert(HashSet::new());
        children.insert(o[1]);
    }
    orbmap
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_make_orb_map() {
        let testmap = String::from("COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L");        
        let m = make_orb_map(&testmap);
        assert_eq!(m[&"COM"], ["B"].iter().cloned().collect()); 
        assert_eq!(m[&"E"], ["J" ,"F"].iter().cloned().collect()); 
        
        assert!(!m.contains_key("L"));  // "L" has nothing in orbit around it
    }
}
