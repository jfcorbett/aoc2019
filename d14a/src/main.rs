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

fn parse_reac_specs(input: &str) -> HashMap<&str, (u32, Vec<(u32, &str)>)> {
    input.lines().map(|line| parse_line(line)).collect()
}

fn main() {
    let wrokfluge = read_whole_file(Path::new("input")).unwrap();
    // let reac_specs:HashMap<_,_> = wrokfluge.lines().map(|line| parse_line(line)).collect();
    let reac_specs = parse_reac_specs(&wrokfluge);
    
    println!("{:?}", reac_specs);
}

fn parse_chem_spec(output: &str) -> (u32, &str) {
    let s:Vec<&str> = output.split(" ").collect();
    (s[0].parse::<u32>().unwrap(), s[1])
}

fn parse_inputs(inputs: &str) -> Vec<(u32, &str)> {
    inputs.split(", ").map(|cs| parse_chem_spec(cs)).collect()
}

fn parse_line(line: &str) -> (&str, (u32, Vec<(u32, &str)>)) {
    let ls = line.split(" => ").collect::<Vec<_>>();
    let out = parse_chem_spec(ls[1]);
    let ins = parse_inputs(ls[0]);
    (out.1, (out.0, ins))
}

fn ore_needed(chem: &mut str, num: u32, reac_specs: &HashMap<&str, (u32, Vec<(u32, &str)>)>, leftovers: &mut HashMap<&str, u32>) -> u32 {
    let mut num_make = num;
    // First check if chem in leftovers; if so, deduct from num and leftovers
    let num_leftover = leftovers.entry(chem).or_insert(0);
    if num_make < *num_leftover {
        // Wow, so many leftovers; Eat from those; No need to cook anything, yay!
        *num_leftover -= num_make;
        return 0;
    } else {
        // Eat all leftovers. Will stil lhave to cook rest, though. Sigh.
        num_make -= *num_leftover;
        *num_leftover = 0;
    }

    if chem == "ORE" {
        return num_make
    } else {
        let (num_out, inputs) = &reac_specs[chem];
        let num_reacs = (num_make - 1) / num_out + 1;        
        *num_leftover += num_make - (num_reacs * num_out);
        return inputs.iter().map(|inp| num_reacs * ore_needed(&mut String::from(inp.1), inp.0, reac_specs, leftovers)).sum::<u32>()
    }
}

// fn yoyo(chem: &str, num: u32, reac_specs: &HashMap<&str, (u32, Vec<(u32, &str)>)>, leftovers: &mut HashMap<&str, u32>) -> u32 {
//     let mut num_make = num;
//     // First check if chem in leftovers; if so, deduct from num and leftovers
//     let num_leftover = leftovers.entry(&String::from(chem)).or_insert(0);
//     if num_make < *num_leftover {
//         // Wow, so many leftovers; Eat from those; No need to cook anything, yay!
//         // *num_leftover -= num_make;
//         return 0;
//     } else {
//         // Eat all leftovers. Will stil lhave to cook rest, though. Sigh.
//         // num_make -= *num_leftover;
//         // *num_leftover = 0;
//     }

//     if chem == "ORE" {
//         return num_make
//     } else {
//         // let (num_out, inputs) = &reac_specs[chem];
//         // let num_reacs = (num_make - 1) / num_out + 1;        
//         // *num_leftover += num_make - (num_reacs * num_out);
//         return 42_u32;  //inputs.iter().map(|inp| num_reacs * yoyo(inp.1, inp.0, reac_specs, leftovers)).sum::<u32>()
//     }
// }

// fn do_thing(key: &mut str, dict: &mut HashMap<&str, u32>) -> u32 {
//     let num = dict.entry(key).or_insert(0);
//     *num += 1;
//     return do_thing(key, dict);
// }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_output() {
        assert_eq!(parse_chem_spec("6 ZSPL"), (6, "ZSPL"));
    }

    #[test]
    fn test_parse_inputs() {
        assert_eq!(parse_inputs("1 HJDM, 1 BMPDP, 8 DRCX, 2 TCTBL, 1 KGWDJ, 16 BRLF, 2 LWPB, 7 KDFQ"),
        vec![(1, "HJDM"), (1, "BMPDP"), (8, "DRCX"), (2, "TCTBL"), (1, "KGWDJ"), (16, "BRLF"), (2, "LWPB"), (7, "KDFQ")]
    );
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("1 HJDM, 1 BMPDP, 8 DRCX, 2 TCTBL, 1 KGWDJ, 16 BRLF, 2 LWPB, 7 KDFQ => 6 ZSPL"),
            ("ZSPL", (6, vec![(1, "HJDM"), (1, "BMPDP"), (8, "DRCX"), (2, "TCTBL"), (1, "KGWDJ"), (16, "BRLF"), (2, "LWPB"), (7, "KDFQ")]))
        )
    }

    #[test]
    fn test_ore_needed_trivial() {
        let reac_specs = parse_reac_specs("91 ORE => 20 FUEL");
        assert_eq!(ore_needed("ORE", 42, &reac_specs, &mut HashMap::new()), 42)
    }

    #[test]
    fn test_ore_needed_almost_trivial() {
        let reac_specs = parse_reac_specs("91 ORE => 20 FUEL");
        assert_eq!(ore_needed("FUEL", 1, &reac_specs), 91);
        assert_eq!(ore_needed("FUEL", 20, &reac_specs), 91);
        assert_eq!(ore_needed("FUEL", 21, &reac_specs), 182);
    }

    #[test]
    fn test_ore_needed_example0() {
        let reac_specs = parse_reac_specs("10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL");
        assert_eq!(ore_needed("FUEL", 1, &reac_specs), 31); 
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
        assert_eq!(ore_needed("BC", 1, &reac_specs), 16); // AB:34 BC:30 CA:16  68+90+64
        // assert_eq!(ore_needed("FUEL", 1, &reac_specs), 165);
    }
    
}