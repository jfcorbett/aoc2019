use std::iter::repeat;
use std::iter::successors;

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
    let input_sequence = parse_input_sequence(&zyrgwunk);
    // let input_sequence = parse_input_sequence("03036732577212944063491565474664");
    println!("{:?}", do_it(&input_sequence));
}

fn do_it(input_sequence: &Vec<i32>) -> String {
    let offset = dbg!(get_offset(input_sequence, 7));
    let mut seq_tail:Vec<i32> = repeat(input_sequence.iter()).take(10_000).flatten().skip(offset).map(|n| *n).collect();
    // repeat_sequence(input_sequence, 10_000)[offset..].iter().rev().map(|n| *n).collect();
    for _ in 0..100 {
        // println!("{:?}", seq_tail[seq_tail.len()-8..].to_vec());
        let mut cs: i32 = 0;
        for i in (0..seq_tail.len()).rev() {
            cs += seq_tail[i];
            cs = cs % 10;
            seq_tail[i] = cs;
        }
    }
    seq_tail.iter().take(8).map(|n| n.to_string()).collect::<Vec<String>>().join("")
    
}

// *************************************************************************************
// Almost everything below this point is useless (except for the parsing bits and pieces)
// *************************************************************************************

fn decode_message(input_sequence: &Vec<i32>) -> String {
    extract_message(&do_phases(repeat_sequence(input_sequence, 10_000), 100), 8, get_offset(input_sequence, 7)) 
}

fn repeat_sequence(sequence: &Vec<i32>, num_repeats: usize) -> Vec<i32> {
    // sequence.iter().map(|n| *n).cycle().take(sequence.len() * num_repeats).collect::<Vec<_>>()
    repeat(sequence.iter()).take(num_repeats).flatten().map(|n| *n).collect::<Vec<_>>()

}

fn extract_message(sequence: &Vec<i32>, msg_len: usize, offset: usize) -> String {
    sequence[offset..(offset+msg_len)].iter().map(|n| n.to_string()).collect::<Vec<String>>().join("")
}

fn get_offset(input_sequence: &Vec<i32>, offset_len: usize) -> usize {
    take_to_string(&input_sequence, offset_len).parse::<usize>().expect("Couldn't parse message offset")
}

fn take_to_string(sequence: &Vec<i32>, num: usize) -> String {
    sequence[0..num].iter().map(|n| n.to_string()).collect::<Vec<String>>().join("")
}

fn do_phases(sequence: Vec<i32>, num_phases: usize) -> Vec<i32> {
    // successors(Some(sequence), |s| Some(do_one_phase(s.to_vec()))).take(num_phases).last().unwrap()
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
        .map(|(n1, n2)| (n1 * n2))
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
    fn test_decode_message() {
        assert_eq!(decode_message(&parse_input_sequence("03036732577212944063491565474664")), "84462026");
    }

    #[test]
    fn test_repeat_sequence() {
        assert_eq!(repeat_sequence(&vec![1,2,3], 4), vec![1,2,3,1,2,3,1,2,3,1,2,3]);
    }

    #[test]
    fn test_extract_message() {
        assert_eq!(extract_message(&parse_input_sequence("98765432109876543210"), 8, 7), "21098765");
    }

    #[test]
    fn test_get_offset() {
        assert_eq!(get_offset(&parse_input_sequence("03036732577212944063491565474664"), 7), 0303673);
    }

    #[test]
    fn test_take_to_string() {
        assert_eq!(take_to_string(&vec![1, 2, 3, 4, 5, 6, 7, 8], 5), "12345");
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
