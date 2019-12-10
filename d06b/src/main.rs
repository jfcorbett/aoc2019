use std::{
    collections::{HashMap},
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
    let pd = make_parent_dict(&whole_file);
    let lineages = ["YOU", "SAN"].iter().map(|x| get_lineage(x, &pd)).collect::<Vec<_>>();
    println!("{:?}", lineages);
}


fn make_parent_dict(orbfile: &str) -> HashMap<String, String> {
    let mut orbmap: HashMap<String, String> = HashMap::new();
    for line in orbfile.lines() {
        let o:Vec<_> = line.split(")").collect();
        orbmap.insert(String::from(o[1]), String::from(o[0]));
    }
    orbmap
}

fn get_lineage(planet: &str, parent_dict: &HashMap<String, String>) -> Vec<String> {
    if !parent_dict.contains_key(&String::from(planet)) {
        vec![String::from(planet)]
    } else {
        let parent = &parent_dict[&String::from(planet)];
        let mut lineage = get_lineage(&parent, &parent_dict);
        lineage.push(String::from(planet));
        lineage
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_lineage() {
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
    let pd = make_parent_dict(&testmap);
    assert_eq!(get_lineage("H", &pd), ["COM","B","G", "H"].iter().map(|x| String::from(*x)).collect::<Vec<_>>());
    }
}