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

// After collecting ORE for a while, you check your cargo hold: 1 trillion (1000000000000) units of ORE.

// With that much ore, given the examples above:

//     The 13312 ORE-per-FUEL example could produce 82892753 FUEL.
//     The 180697 ORE-per-FUEL example could produce 5586022 FUEL.
//     The 2210736 ORE-per-FUEL example could produce 460664 FUEL.

const ORE_RESERVES: u64 = 1_000_000_000_000;

fn main() {
    let wrokfluge = read_whole_file(Path::new("input")).unwrap();
    let reac_specs = parse_reac_specs(&wrokfluge);
    
    let mut leftovers = HashMap::new();
    println!("{}", ore_needed("FUEL", 1, &reac_specs, &mut leftovers));
}

fn max_num_produceable(chem: &str, num_ore: u64, reac_specs: &HashMap<String, (u64, Vec<(u64, String)>)>) -> u64 {
    let lb = ORE_RESERVES / ore_needed(chem, 1, &reac_specs, &mut HashMap::new());
    let mut num_chem_prev: u64 = lb;
    loop {
        let try_num_chem = num_chem_prev + 1;
        if ore_needed(chem, try_num_chem, &reac_specs, &mut HashMap::new()) > ORE_RESERVES {
            return num_chem_prev;
        };
        num_chem_prev = try_num_chem;
    }
}

fn ore_needed(chem: &str, num: u64, reac_specs: &HashMap<String, (u64, Vec<(u64, String)>)>, leftovers: &mut HashMap<String, u64>) -> u64 {
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
        return inputs.iter().map(|inp| ore_needed(&inp.1, num_reacs*inp.0, reac_specs, leftovers)).sum::<u64>()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_num_produceable_example2() {
        let reac_specs = parse_reac_specs("157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT");        
        assert_eq!(max_num_produceable("FUEL", ORE_RESERVES, &reac_specs), 82892753);
    }

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
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT");
        assert_eq!(ore_needed("FUEL", 1, &reac_specs, &mut HashMap::new()), 13312);
    }

    
}

fn parse_reac_specs(input: &str) -> HashMap<String, (u64, Vec<(u64, String)>)> {
    input.lines().map(|line| parse_line(line)).collect()
}

fn parse_chem_spec(output: &str) -> (u64, String) {
    let s:Vec<String> = output.split(" ").map(|s| String::from(s)).collect();
    (s[0].parse::<u64>().unwrap(), String::from(&s[1]))
}

fn parse_inputs(inputs: &str) -> Vec<(u64, String)> {
    inputs.split(", ").map(|cs| parse_chem_spec(cs)).collect()
}

fn parse_line(line: &str) -> (String, (u64, Vec<(u64, String)>)) {
    let ls = line.split(" => ").collect::<Vec<_>>();
    let out = parse_chem_spec(ls[1]);
    let ins = parse_inputs(ls[0]);
    (out.1, (out.0, ins))
}

