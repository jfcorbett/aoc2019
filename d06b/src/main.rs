use std::{
    collections::{HashMap},
};

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
    let whole_file = read_whole_file(Path::new("input")).unwrap(); // String
    let pd = make_parent_dict(&whole_file);
    let lineages = ["YOU", "SAN"].iter().map(|x| get_lineage(x, &pd)).collect::<Vec<_>>();
    let mut num_common = 0;
    for (i, l) in lineages[0].iter().enumerate() {
        if String::from(l) != lineages[1][i] {
            num_common = i as u32;
            break;
        }
    }

    // count each branch from the point they diverge
    // add the two
    println!("l0:{} l1:{} common:{}", lineages[0].len(), lineages[1].len(), num_common);
    print!("{}", lineages[0].len() as u32 + lineages[1].len() as u32 - 2*num_common - 2);
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