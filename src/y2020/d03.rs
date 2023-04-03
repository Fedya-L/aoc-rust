use std::collections::HashSet;

mod tests {
    use super::*;

    #[test]
    fn test_hash_set() {
        let mut hs: HashSet<[usize; 2]> = HashSet::new();
        hs.insert([0, 0]);
        hs.insert([1, 1]);

        for i in 0..2 {
            for ii in 0..2 {
                let c = [i, ii];
                println!("Does {:?} exist? {}", c, hs.contains(&c));
                assert_eq!(hs.contains(&c), c[0] == c[1]);
            }
        }
    }
}
