

// An Elf just remembered one more important detail: the two adjacent matching digits are not part of a larger group of matching digits.

// Given this additional criterion, but still ignoring the range rule, the following are now true:

//     112233 meets these criteria because the digits never decrease and all repeated digits are exactly two digits long.
//     123444 no longer meets the criteria (the repeated 44 is part of a larger group of 444).
//     111122 meets the criteria (even though 1 is repeated more than twice, it still contains a double 22).


fn main() {
    let n0 = 128392;
    let n1 = 643281;
    // let n0 = 123454;
    // let n1 = 123471;

    let maybe = (n0..n1)
        .map(|n| n.to_string().into_bytes())
        .filter(|ns| gets_angrier(&ns) && sequence_lengths(&ns).contains(&2));
    // println!("yo.");
    println!("{:?}", maybe.count());
}

fn gets_angrier(ns: &Vec<u8>) -> bool {
    // Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
    ns.windows(2).all(|n| n[0] <= n[1])
}

fn sequence_lengths(ns: &Vec<u8>) -> Vec<i32> {
    let mut prev = 0u8;
    let mut sl = 1i32;
    let mut sls = vec![]; 
    for (i, n) in ns.iter().enumerate() {
        if i == 0 {
            prev = *n;            
        } else if *n == prev {
            sl += 1;
        } else {
            prev = *n;
            sls.push(sl);
            sl = 1;
        }

        if i == ns.len()-1 {
            sls.push(sl);
        }
    }
    sls
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gets_angrier() {
        assert!(gets_angrier(&111123.to_string().into_bytes()));
        assert!(gets_angrier(&135679.to_string().into_bytes()));
        assert!(gets_angrier(&111111.to_string().into_bytes()));

        assert!(!gets_angrier(&223450.to_string().into_bytes()));
    }

    #[test]
    fn test_sequence_lengths() {
        assert_eq!(vec![6], sequence_lengths(&111111.to_string().into_bytes()));
        assert_eq!(vec![2,2,2], sequence_lengths(&112233.to_string().into_bytes()));
        assert_eq!(vec![1,1,1,3], sequence_lengths(&123444.to_string().into_bytes()));
        assert_eq!(vec![4,2], sequence_lengths(&111122.to_string().into_bytes()));

    }
}

