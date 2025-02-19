use rand::prelude::*;
use rand::distr::weighted::*;

/// The numbers of characters in the markov chain.
const OCHARS: usize = 3;

/// The weights for the first character of the markov chain.
const FIRST: [u8; OCHARS] = [4,2,1];

/// The weights for the gagging markov chain.
const NEXT: [[u8; OCHARS]; OCHARS] = [
//   h m f
    [2,4,1], // h
    [1,4,2], // m
    [2,1,4]  // f
];

/// The lowercase letters for the markov chain to return.
const LOW: [char; OCHARS] = ['h', 'm', 'f'];
/// The uppercase letters for the markov chain to return.
const CAP: [char; OCHARS] = ['H', 'M', 'F'];

/// A basic markov-chain based gagging algorithm.
///
/// It replaces all letter characters with any of `h`, `m`, and `f`, preserving case.
pub fn gag(s: &str) -> String {
    let mut ret = String::with_capacity(s.len());
    let mut rng = rand::rng();
    let get_first = WeightedIndex::new(FIRST).expect("The weights for getting the first character to be valid.");
    let mut ochar = get_first.sample(&mut rng);
    for c in s.chars() {
        if c.is_ascii_alphabetic() {
            ret.push(if c.is_ascii_uppercase() {CAP} else {LOW}[ochar]);
            ochar = WeightedIndex::new(NEXT[ochar]).expect("The weights forg getting the next character to be valid.").sample(&mut rng);
        } else {
            ret.push(c);
            ochar = get_first.sample(&mut rng);
        }
    }
    ret
}
