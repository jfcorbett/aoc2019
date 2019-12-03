use std::fs::File;
use std::io::Read;

fn main() {

    // Read file lines
    let input_file = String::from(".\\input.txt");
    let mut data = String::new();
    let mut f = File::open(input_file).expect("Unable to open file");
    f.read_to_string(&mut data).expect("Unable to read string");
    let lines = data.split("\n");

    // let mut masses = vec![];
    let mut sum_fuel = 0i32;
    for s in lines {
        let s_tmp = s.trim();
        if s_tmp.len() > 0 {
            let x = s.trim().parse::<i32>().unwrap();
            // masses.push(x);
            sum_fuel += fuel_req(x)
            // println!("{}", x);
        }
    }
    println!("{}", sum_fuel);
}

fn fuel_req(mass: i32) -> i32 {
    if mass < 9 {
        0
    } else {
        mass / 3 - 2 + fuel_req(mass / 3 - 2)  // fuel for the fuel
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_fuel_req() {
        assert_eq!(fuel_req(12), 2);
        assert_eq!(fuel_req(14), 2);
        assert_eq!(fuel_req(1969), 966);
        assert_eq!(fuel_req(100756), 50346);
    }
}