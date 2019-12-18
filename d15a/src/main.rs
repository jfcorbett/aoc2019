use std::iter::repeat;

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
    let zyrgwunk = read_whole_file(Path::new("input")).unwrap();
    let input = parse_input_sequence(&zyrgwunk);
    println!("{}", take_to_string(do_phases(input, 100), 8));
}

fn take_to_string(sequence: Vec<i32>, num: usize) -> String {
    sequence[0..num].iter().map(|n| n.to_string()).collect::<Vec<String>>().join("")
}

fn do_phases(sequence: Vec<i32>, num_phases: usize) -> Vec<i32> {
    if num_phases < 1 {
        return sequence;
    }
    do_one_phase(do_phases(sequence, num_phases - 1))
}

fn do_one_phase(sequence: Vec<i32>) -> Vec<i32> {
    (0..sequence.len())
        .map(|n| weird_mult(&sequence, &wave(n + 1, sequence.len())))
        .collect()
}

fn weird_mult(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    v1.iter()
        .zip(v2.iter())
        .map(|(n1, n2)| n1 * n2)
        .sum::<i32>()
        .abs()
        % 10
}

fn wave(freq: usize, length: usize) -> Vec<i32> {
    // Make wave!
    vec![0, 1, 0, -1]
        .iter()
        .flat_map(|n| repeat(*n).take(freq))
        .cycle()
        .skip(1)
        .take(length)
        .collect()
}

fn parse_input_sequence(in_str: &str) -> Vec<i32> {
    in_str
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_take_to_string() {
        assert_eq!(take_to_string(vec![1, 2, 3, 4, 5, 6, 7, 8], 5), "12345");
    }

    #[test]
    fn test_do_phases() {
        assert_eq!(
            do_phases(vec![1, 2, 3, 4, 5, 6, 7, 8], 0),
            vec![1, 2, 3, 4, 5, 6, 7, 8]
        );
        assert_eq!(
            do_phases(vec![1, 2, 3, 4, 5, 6, 7, 8], 1),
            vec![4, 8, 2, 2, 6, 1, 5, 8]
        );
        assert_eq!(
            do_phases(vec![1, 2, 3, 4, 5, 6, 7, 8], 2),
            vec![3, 4, 0, 4, 0, 4, 3, 8]
        );
        assert_eq!(
            do_phases(vec![1, 2, 3, 4, 5, 6, 7, 8], 3),
            vec![0, 3, 4, 1, 5, 5, 1, 8]
        );

        assert_eq!(
            do_phases(vec![4, 8, 2, 2, 6, 1, 5, 8], 1),
            vec![3, 4, 0, 4, 0, 4, 3, 8]
        );

        assert_eq!(do_phases(parse_input_sequence("80871224585914546619083218645595"), 100)[0..8],
            vec![2,4,1,7,6,1,7,6][..]);
        assert_eq!(do_phases(parse_input_sequence("19617804207202209144916044189917"), 100)[0..8],
            vec![7,3,7,4,5,4,1,8][..]);
        assert_eq!(do_phases(parse_input_sequence("69317163492948606335995924319873"), 100)[0..8],
            vec![5,2,4,3,2,1,3,3][..]);
    }

    #[test]
    fn test_do_one_phase() {
        assert_eq!(
            do_one_phase(vec![1, 2, 3, 4, 5, 6, 7, 8]),
            vec![4, 8, 2, 2, 6, 1, 5, 8]
        );
        assert_eq!(
            do_one_phase(vec![4, 8, 2, 2, 6, 1, 5, 8]),
            vec![3, 4, 0, 4, 0, 4, 3, 8]
        );
    }

    #[test]
    fn test_weird_mult() {
        assert_eq!(weird_mult(&vec![1, 2, 3, 4, 5, 6, 7, 8], &wave(1, 8)), 4);
        assert_eq!(weird_mult(&vec![1, 2, 3, 4, 5, 6, 7, 8], &wave(2, 8)), 8);
        assert_eq!(weird_mult(&vec![1, 2, 3, 4, 5, 6, 7, 8], &wave(3, 8)), 2);
        assert_eq!(weird_mult(&vec![1, 2, 3, 4, 5, 6, 7, 8], &wave(4, 8)), 2);
        assert_eq!(weird_mult(&vec![1, 2, 3, 4, 5, 6, 7, 8], &wave(5, 8)), 6);
        assert_eq!(weird_mult(&vec![1, 2, 3, 4, 5, 6, 7, 8], &wave(6, 8)), 1);
        assert_eq!(weird_mult(&vec![1, 2, 3, 4, 5, 6, 7, 8], &wave(7, 8)), 5);
        assert_eq!(weird_mult(&vec![1, 2, 3, 4, 5, 6, 7, 8], &wave(8, 8)), 8);
    }
    #[test]
    fn test_wave() {
        assert_eq!(wave(1, 5), vec![1, 0, -1, 0, 1]);
        assert_eq!(wave(2, 9), vec![0, 1, 1, 0, 0, -1, -1, 0, 0]);
        assert_eq!(
            wave(3, 15),
            vec![0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1, 0, 0, 0, 1]
        );
    }

    #[test]
    fn test_parse_input_sequence() {
        assert_eq!(
            parse_input_sequence("12345678"),
            vec![1, 2, 3, 4, 5, 6, 7, 8]
        );
    }
}
