
fn main() {
    let n0 = 128392;
    let n1 = 643281;
    // let n0 = 123454;
    // let n1 = 123471;

    let maybe = (n0..n1)
        .map(|n| n.to_string().into_bytes())
        .filter(|ns| foams_at_mouth(&ns));
    // println!("yo.");
    println!("{:?}", maybe.count());
}

fn stutters(ns: &Vec<u8>) -> bool {
    // Two adjacent digits are the same (like 22 in 122345).
    ns.windows(2).any(|n| n[0] == n[1])
}

fn gets_angrier(ns: &Vec<u8>) -> bool {
    // Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
    ns.windows(2).all(|n| n[0] <= n[1])
}

fn foams_at_mouth(ns: &Vec<u8>) -> bool {
    stutters(ns) && gets_angrier(ns)
    // false
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stutters() {
        assert!(stutters(&122345.to_string().into_bytes())); 
        assert!(stutters(&111123.to_string().into_bytes())); 
        assert!(stutters(&111111.to_string().into_bytes())); 
        
        assert!(!stutters(&123789.to_string().into_bytes()));
        assert!(!stutters(&135679.to_string().into_bytes()));
    }

    #[test]
    fn test_gets_angrier() {
        assert!(gets_angrier(&111123.to_string().into_bytes()));
        assert!(gets_angrier(&135679.to_string().into_bytes()));
        assert!(gets_angrier(&111111.to_string().into_bytes()));

        assert!(!gets_angrier(&223450.to_string().into_bytes()));
    }

    #[test]
    fn test_foams_at_mouth() {
        assert!(foams_at_mouth(&111111.to_string().into_bytes()));
        assert!(foams_at_mouth(&122345.to_string().into_bytes()));
        assert!(foams_at_mouth(&111123.to_string().into_bytes()));

        assert!(!foams_at_mouth(&223450.to_string().into_bytes()));
        assert!(!foams_at_mouth(&123789.to_string().into_bytes()));
    }
}

