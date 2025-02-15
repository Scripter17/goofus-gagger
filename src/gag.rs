use rand::prelude::*;
use rand::distr::weighted::*;

const OCHARS: usize = 3;

const FIRST: [u8; OCHARS] = [4,2,1];

const NEXT: [[u8; OCHARS]; OCHARS] = [
//   h m f
    [2,4,1], // h
    [1,4,2], // m
    [2,1,4]  // f
];

const LOW: [char; OCHARS] = ['h', 'm', 'f'];
const CAP: [char; OCHARS] = ['H', 'M', 'F'];

pub fn gag(s: &str) -> String {
    let mut ret = String::with_capacity(s.len());
    let mut rng = rand::rng();
    let get_first = WeightedIndex::new(FIRST).unwrap();
    let mut ochar = get_first.sample(&mut rng);
    for c in s.chars() {
        if c.is_ascii_alphabetic() {
            ret.push(if c.is_ascii_uppercase() {CAP} else {LOW}[ochar]);
            ochar = WeightedIndex::new(NEXT[ochar]).unwrap().sample(&mut rng);
        } else {
            ret.push(c);
            ochar = get_first.sample(&mut rng);
        }
    }
    ret
}
