use std::str::FromStr;

use rand::prelude::*;
use rand::distr::weighted::*;
use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, poise::macros::ChoiceParameter)]
pub enum RewriterName {
    #[default]
    Gag,
    Dog,
    Cow,
    Fox
}

#[derive(Debug, Error)]
#[error("Unknwon RewriterName")]
pub struct UnknownRewriterName;

impl FromStr for RewriterName {
    type Err = UnknownRewriterName;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gag" => Ok(Self::Gag),
            "dog" => Ok(Self::Dog),
            "cow" => Ok(Self::Cow),
            "fox" => Ok(Self::Fox),
            _ => Err(UnknownRewriterName)
        }
    }
}

impl RewriterName {
    pub fn get(&self) -> &'static Rewriter {
        match self {
            Self::Gag => &GAG_REWRITER,
            Self::Dog => &DOG_REWRITER,
            Self::Cow => todo!(),
            Self::Fox => todo!()
        }
    }
}

pub const REWRITER_STATES: usize = 16;

pub struct Rewriter {
    pub chars: [char; REWRITER_STATES],
    pub first: [u8; REWRITER_STATES],
    pub next: [[u8; REWRITER_STATES]; REWRITER_STATES]
}

#[derive(Debug, Error)]
pub enum RewriterError {
    #[error(transparent)]
    WeightError(#[from] rand::seq::WeightError)
}

impl Rewriter {
    pub fn rewrite(&self, text: &str) -> Result<String, RewriterError> {
        let mut ret = String::with_capacity(text.len());
        let mut rng = rand::rng();
        let get_first = WeightedIndex::new(self.first)?;
        let mut ochar = get_first.sample(&mut rng);
        for c in text.chars() {
            if c.is_alphabetic() {
                ret.push_str(&if c.is_uppercase() {self.chars[ochar].to_uppercase().to_string()} else {self.chars[ochar].to_lowercase().to_string()});
                ochar = WeightedIndex::new(self.next[ochar])?.sample(&mut rng);
            } else {
                ret.push(c);
                ochar = get_first.sample(&mut rng);
            }
        }
        Ok(ret)
    }
}

pub const EMPTY_REWRITER: Rewriter = Rewriter {
    chars: [' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' '],
    first: [ 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
    next: [
           [ 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ]
    ]
};

pub const GAG_REWRITER: Rewriter = Rewriter {
    chars: ['h','m','f' , ' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' '],
    first: [ 4 , 2 , 1  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
    next: [
           [ 2 , 4 , 1  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 1 , 4 , 2  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 2 , 1 , 4  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],

           [ 0 , 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ]
    ]
};

pub const DOG_REWRITER: Rewriter = Rewriter {
    chars: ['a','w','r','u','f' , 'w','o','f' , 'b','a','r','k','!' , 'a','w','o'],
    first: [ 4 , 2,  2 , 0 , 0  ,  4 , 0 , 0  ,  1 , 0 , 0 , 0 , 0  ,  1 , 0 , 0 ],
    next: [
           [ 1 , 2 , 2 , 1 , 0  ,  0 , 0 , 0  ,  0 , 0 , 0 , 0 , 0  ,  0 , 0 , 0 ],
           [ 1 , 1 , 2 , 2 , 0  ,  0 , 0 , 0  ,  0 , 0 , 0 , 0 , 0  ,  0 , 0 , 0 ],
           [ 1 , 1 , 2 , 4 , 4  ,  0 , 0 , 0  ,  0 , 0 , 0 , 0 , 0  ,  0 , 0 , 0 ],
           [ 1 , 1 , 1 , 1 , 4  ,  0 , 0 , 0  ,  0 , 0 , 0 , 0 , 0  ,  0 , 0 , 0 ],
           [ 1 , 1 , 2 , 1 , 2  ,  0 , 0 , 0  ,  0 , 0 , 0 , 0 , 0  ,  0 , 0 , 0 ],

           [ 0 , 0 , 0 , 0 , 0  ,  1 , 4 , 0  ,  0 , 0 , 0 , 0 , 0  ,  0 , 0 , 0 ],
           [ 0 , 0 , 0 , 0 , 0  ,  1 , 4 , 2  ,  0 , 0 , 0 , 0 , 0  ,  0 , 0 , 0 ],
           [ 0 , 0 , 0 , 0 , 0  ,  0 , 0 , 1  ,  0 , 0 , 0 , 0 , 0  ,  0 , 0 , 0 ],

           [ 0 , 0 , 0 , 0 , 0  ,  0 , 0 , 0  ,  0 , 4 , 0 , 0 , 0  ,  0 , 0 , 0 ],
           [ 0 , 0 , 0 , 0 , 0  ,  0 , 0 , 0  ,  0 , 1 , 2 , 0 , 0  ,  0 , 0 , 0 ],
           [ 0 , 0 , 0 , 0 , 0  ,  0 , 0 , 0  ,  0 , 0 , 1 , 1 , 0  ,  0 , 0 , 0 ],
           [ 0 , 0 , 0 , 0 , 0  ,  0 , 0 , 0  ,  0 , 0 , 1 , 1 , 1  ,  0 , 0 , 0 ],
           [ 0 , 0 , 0 , 0 , 0  ,  0 , 0 , 0  ,  0 , 0 , 0 , 0 , 1  ,  0 , 0 , 0 ],

           [ 0 , 0 , 0 , 0 , 0  ,  0 , 0 , 0  ,  0 , 0 , 0 , 0 , 0  ,  1 , 1 , 0 ],
           [ 0 , 0 , 0 , 0 , 0  ,  0 , 0 , 0  ,  0 , 0 , 0 , 0 , 0  ,  0 , 1 , 2 ],
           [ 0 , 0 , 0 , 0 , 0  ,  0 , 0 , 0  ,  0 , 0 , 0 , 0 , 0  ,  1 , 1 , 4 ]
    ]
};
