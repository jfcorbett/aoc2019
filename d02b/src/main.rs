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

    'nl: for noun in 0..99 {
        'vl: for verb in 0..99 {
            let mut prog = iz.clone();
            prog[1] = noun;
            prog[2] = verb;
            if run_prog(&prog)[0] == 19690720 {
                println!("{}", 100 * noun + verb);
                break 'nl;
            }

        }
    }
}

fn run_prog(prog: &Vec<i32>) -> Vec<i32> {
    let mut iz = prog.clone();
    let mut pos = 0;
    while iz[pos] != 99 {
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
    iz
}

fn progs_equal(a: &[i32], b: &[i32]) -> bool {
    (a.len() == b.len()) && a.iter().zip(b).all(|(a, b)| a == b)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_progs_equal() {
        assert!(progs_equal(&vec![1, 0, 0, 0, 99], &vec![1, 0, 0, 0, 99]));  // same! yay!
        
        assert!(!progs_equal(&vec![1, 0, 0, 0, 99], &vec![1, 666, 0, 0, 99]));  // one diff
        assert!(!progs_equal(&vec![1, 0, 0, 0, 99], &vec![1, 0, 0, 0]));  // unequal lengths
    }

    #[test]
    fn test_run_prog() {
        assert!(progs_equal(
            &run_prog(&vec![1, 0, 0, 0, 99]),
            &vec![2, 0, 0, 0, 99]
        ));
        assert!(progs_equal(
            &run_prog(&vec![2, 3, 0, 3, 99]),
            &vec![2, 3, 0, 6, 99]
        ));
        assert!(progs_equal(
            &run_prog(&vec![2, 4, 4, 5, 99, 0]),
            &vec![2, 4, 4, 5, 99, 9801]
        ));
        assert!(progs_equal(
            &run_prog(&vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
            &vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        ));
    }
}
