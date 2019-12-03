use std::fs::File;
use std::io::Read;

fn main() {
    let input_file = String::from("input.txt");
    let mut file_contents = String::new();
    let mut f = File::open(input_file).expect("Unable to open file");
    f.read_to_string(&mut file_contents)
        .expect("Unable to read string");
    let atoms = file_contents.split(",");

    let mut iz = vec![];
    for s in atoms {
        let s_tmp = s.trim();
        if s_tmp.len() > 0 {
            let intcode = s.trim().parse::<i32>().unwrap();
            iz.push(intcode);
            // println!("{}", intcode);
        }
    }

    // 1202 program alarm -- gravity assist program
    iz[1] = 12;
    iz[2] = 2;

    run_prog(&mut iz);
    println!("{:?}", iz);
}

fn run_prog(iz: &mut Vec<i32>) {
    let mut pos = 0;
    while iz[pos] != 99 {
        // println!("{}", intcodez[pos]);
        let target = iz[pos + 3] as usize;
        let i1 = iz[pos + 1] as usize;
        let i2 = iz[pos + 2] as usize;
        if iz[pos] == 1 {
            iz[target] = iz[i1] + iz[i2];
        } else if iz[pos] == 2 {
            iz[target] = iz[i1] * iz[i2];
        } else {
            panic!("WTF is this: {}", iz[pos]);
        }
        pos += 4;
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_run_prog() {
        let mut a = vec![1, 0, 0, 0, 99];
        let b = vec![2, 0, 0, 0, 99];
        run_prog(&mut a);
        assert!(a.iter().zip(&b).all(|(a, b)| a == b));

        let mut a = vec![2,3,0,3,99];
        let b = vec![2,3,0,6,99];
        run_prog(&mut a);
        assert!(a.iter().zip(&b).all(|(a, b)| a == b));
        
        let mut a = vec![2,4,4,5,99,0];
        let b = vec![2,4,4,5,99,9801];
        run_prog(&mut a);
        assert!(a.iter().zip(&b).all(|(a, b)| a == b));
        
        let mut a = vec![1,1,1,4,99,5,6,0,99];
        let b = vec![30,1,1,4,2,5,6,0,99];
        run_prog(&mut a);
        assert!(a.iter().zip(&b).all(|(a, b)| a == b));
    }
}
