use std::collections::HashMap;

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

fn parse_reac_specs(input: &str) -> HashMap<String, (u32, Vec<(u32, String)>)> {
    input.lines().map(|line| parse_line(line)).collect()
}

fn main() {
    // let wrokfluge = read_whole_file(Path::new("input")).unwrap();
    // let reac_specs = parse_reac_specs(&wrokfluge);
    let reac_specs = parse_reac_specs("9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL");
    let mut leftovers = HashMap::new();
    println!("{}", ore_needed("FUEL", 1, &reac_specs, &mut leftovers));

    println!("{:?}", leftovers);
}

fn parse_chem_spec(output: &str) -> (u32, String) {
    let s:Vec<String> = output.split(" ").map(|s| String::from(s)).collect();
    (s[0].parse::<u32>().unwrap(), String::from(&s[1]))
}

fn parse_inputs(inputs: &str) -> Vec<(u32, String)> {
    inputs.split(", ").map(|cs| parse_chem_spec(cs)).collect()
}

fn parse_line(line: &str) -> (String, (u32, Vec<(u32, String)>)) {
    let ls = line.split(" => ").collect::<Vec<_>>();
    let out = parse_chem_spec(ls[1]);
    let ins = parse_inputs(ls[0]);
    (out.1, (out.0, ins))
}

fn ore_needed(chem: &str, num: u32, reac_specs: &HashMap<String, (u32, Vec<(u32, String)>)>, leftovers: &mut HashMap<String, u32>) -> u32 {
    let mut num_needed = num;
    // First check if chem in leftovers; if so, deduct from num and leftovers
    let num_leftover = leftovers.entry(String::from(chem)).or_insert(0);
    if num_needed <= *num_leftover {
        // Wow, so many leftovers; Eat from those; No need to cook anything, yay!
        *num_leftover -= num_needed;
        return 0;
    } else {
        // Eat all leftovers. Will stil lhave to cook rest, though. Sigh.
        num_needed -= *num_leftover;
        *num_leftover = 0;
    }

    if chem == "ORE" {
        return num_needed
    } else {
        let (num_out, inputs) = &reac_specs[chem];
        let num_reacs = (num_needed - 1) / num_out + 1;        
        *num_leftover += (num_reacs * num_out) - num_needed; 
        return inputs.iter().map(|inp| num_reacs * ore_needed(&inp.1, inp.0, reac_specs, leftovers)).sum::<u32>()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_chem_spec() {
        assert_eq!(parse_chem_spec("6 ZSPL"), (6, String::from("ZSPL")));
    }

    // #[test]
    // fn test_parse_inputs() {
    //     assert_eq!(parse_inputs("1 HJDM, 1 BMPDP, 8 DRCX, 2 TCTBL, 1 KGWDJ, 16 BRLF, 2 LWPB, 7 KDFQ"),
    //     vec![(1, "HJDM"), (1, "BMPDP"), (8, "DRCX"), (2, "TCTBL"), (1, "KGWDJ"), (16, "BRLF"), (2, "LWPB"), (7, "KDFQ")]
    // );
    // }

    // #[test]
    // fn test_parse_line() {
    //     assert_eq!(
    //         parse_line("1 HJDM, 1 BMPDP, 8 DRCX, 2 TCTBL, 1 KGWDJ, 16 BRLF, 2 LWPB, 7 KDFQ => 6 ZSPL"),
    //         ("ZSPL", (6, vec![(1, "HJDM"), (1, "BMPDP"), (8, "DRCX"), (2, "TCTBL"), (1, "KGWDJ"), (16, "BRLF"), (2, "LWPB"), (7, "KDFQ")]))
    //     )
    // }

    #[test]
    fn test_ore_needed_trivial() {
        let reac_specs = parse_reac_specs("91 ORE => 20 FUEL");
        assert_eq!(ore_needed("ORE", 42, &reac_specs, &mut HashMap::new()), 42)
    }

    #[test]
    fn test_ore_needed_almost_trivial() {
        let reac_specs = parse_reac_specs("91 ORE => 20 FUEL");
        assert_eq!(ore_needed("FUEL", 1, &reac_specs, &mut HashMap::new()), 91);
        assert_eq!(ore_needed("FUEL", 20, &reac_specs, &mut HashMap::new()), 91);
        assert_eq!(ore_needed("FUEL", 21, &reac_specs, &mut HashMap::new()), 182);
    }

    #[test]
    fn test_ore_needed_example0() {
        let reac_specs = parse_reac_specs("10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL");
        assert_eq!(ore_needed("FUEL", 1, &reac_specs, &mut HashMap::new()), 31); 
    }

    #[test]
    fn test_ore_needed_example1() {
        let reac_specs = parse_reac_specs("9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL");
        // assert_eq!(ore_needed("BC", 1, &reac_specs, &mut HashMap::new()), 16); 
        assert_eq!(ore_needed("FUEL", 1, &reac_specs, &mut HashMap::new()), 165);
    }

    #[test]
    fn test_ore_needed_example2() {
        let reac_specs = parse_reac_specs("157 ORE => 5 NZVS
7 DCFZ, 7 PSHF => 2 XJWVT
179 ORE => 7 PSHF
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
177 ORE => 5 HKGWZ
165 ORE => 2 GPVTF
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT");
        assert_eq!(ore_needed("FUEL", 1, &reac_specs, &mut HashMap::new()), 13312);
    }

    
}