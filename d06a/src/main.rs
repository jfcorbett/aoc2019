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
    let n = count_orbits_around("COM", 1, &m);
    println!("{:?}", n);
}

fn count_orbits_around(planet: &str, depth: u32, orbmap: &HashMap<String, HashSet<String>>) -> u32 {
    if !orbmap.contains_key(&String::from(planet)) {
        0
    } else {
        let direct_orbits = &orbmap[&String::from(planet)];
        let num_direct_orbits = (direct_orbits.len() as u32) * depth;
        let mut num_indirect_orbits = 0_u32;
        for d in direct_orbits {
            num_indirect_orbits += count_orbits_around(&d, depth+1, orbmap);
        }
        num_direct_orbits + num_indirect_orbits
    }
}

fn make_orb_map(orbfile: &str) -> HashMap<String, HashSet<String>> {
    let mut orbmap: HashMap<String, HashSet<String>> = HashMap::new();
    for line in orbfile.lines() {
        let o:Vec<_> = line.split(")").collect();
        let children = orbmap.entry(String::from(o[0])).or_insert(HashSet::new());
        children.insert(String::from(o[1]));
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
        assert_eq!(m["COM"], ["B"].iter().map(|x| String::from(*x)).collect()); 
        assert_eq!(m["E"], ["J" ,"F"].iter().map(|x| String::from(*x)).collect()); 
        
        assert!(!m.contains_key("L"));  // "L" has nothing in orbit around it
    }

    #[test]
    fn test_count_orbits_around() {
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
        assert_eq!(count_orbits_around("COM", 1, &m), 42);
    }
}
