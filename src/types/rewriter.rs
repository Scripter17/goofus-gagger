//! The code to rewrite/gag messages.

use std::str::FromStr;
use std::collections::HashSet;

use rand::prelude::*;
use rand::distr::weighted::*;
use serde::{Serialize, Deserialize};
use thiserror::Error;
use poise::ChoiceParameter;

/// The name of a [`GagMode`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize, ChoiceParameter)]
pub enum GagModeName {
    /// A gag.
    #[default]
    Gag,
    /// Makes you sound like [`Self::Gag`].
    Sock,
    /// Makes you sound like a dog.
    Dog,
    /// Makes you sound like [`Self::Dog`].
    Puppy,
    /// Makes you sound like a cow.
    Cow,
    /// Makes you sound like a fox.
    Fox,
    /// Makes you sound like a cat.
    Cat,
    /// Makes you sound like a seal.
    Seal,
    /// Makes you sound like a bee.
    Bee
}

impl GagModeName {
    /// A [`HashSet`] with all [`GagModeName`]s.
    pub fn all() -> HashSet<Self> {
        Self::list().into_iter().map(|x| Self::from_name(&x.name).expect("ChoiceParameter to be implemented correctly")).collect()
    }

    /// The icon of a [`GagModeName`]. Usually an emoji.
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Gag   => "🔴",
            Self::Sock  => "🧦",
            Self::Dog   => "🐶",
            Self::Puppy => "🐶",
            Self::Cow   => "🐮",
            Self::Fox   => "🦊",
            Self::Cat   => "🐱",
            Self::Seal  => "🦭",
            Self::Bee   => "🐝"
        }
    }

    /// Gets the [`GagMode`].
    pub fn get(&self) -> &'static GagMode {
        match self {
            Self::Gag   => &GAG_GAGMODE,
            Self::Sock  => &GAG_GAGMODE,
            Self::Dog   => &DOG_GAGMODE,
            Self::Puppy => &DOG_GAGMODE,
            Self::Cow   => &COW_GAGMODE,
            Self::Fox   => &FOX_GAGMODE,
            Self::Cat   => &CAT_GAGMODE,
            Self::Seal  => &SEAL_GAGMODE,
            Self::Bee   => &BEE_GAGMODE
        }
    }
}

/// Unknown [`GagModeName`]
#[derive(Debug, Error)]
#[error("Unknwon GagModeName")]
pub struct UnknownGagModeName;

impl FromStr for GagModeName {
    type Err = UnknownGagModeName;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_name(s).ok_or(UnknownGagModeName)
    }
}

impl std::fmt::Display for GagModeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(self.name())
    }
}

/// The number of states a [`GagMode`] markov chain can have.
pub const GAGMODE_STATES: usize = 16;

/// A markov chain-based gag mode to rewrite messages.
pub struct GagMode {
    /// The character to output for each state.
    pub chars: [char; GAGMODE_STATES],
    /// The weights of each state to start each word with.
    pub first: [u8; GAGMODE_STATES],
    /// The weights of each next state.
    ///
    /// `weight_for_next_state = x[current_state][possible_next_state]`.
    pub next: [[u8; GAGMODE_STATES]; GAGMODE_STATES]
}

/// The enum of errors [`GagMode::rewrite`] can return.
#[derive(Debug, Error)]
pub enum GagModeError {
    /// Tried to give [`rand`] invalid weights.
    #[error(transparent)]
    WeightError(#[from] rand::seq::WeightError)
}

impl GagMode {
    /// Rewrite a message.
    pub fn rewrite(&self, text: &str) -> Result<String, GagModeError> {
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

/// The empty [`GagMode`]. Used for copy-pasting.
#[allow(dead_code, reason = "Used for copy pasting")]
pub const EMPTY_GAGMODE: GagMode = GagMode {
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

/// [`GagModeName::Gag`].
pub const GAG_GAGMODE: GagMode = GagMode {
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

/// [`GagModeName::Dog`]
pub const DOG_GAGMODE: GagMode = GagMode {
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

/// [`GagModeName::Cow`]
pub const COW_GAGMODE: GagMode = GagMode {
    chars: ['m','o' , ' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' '],
    first: [ 1 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
    next: [
           [ 1 , 1  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 1  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],

           [ 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ]
    ]
};

/// [`GagModename::Fox`]
pub const FOX_GAGMODE: GagMode = GagMode {
    chars: ['a','e','h' , ' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' '],
    first: [ 1 , 1 , 1  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
    next: [
           [ 1 , 1 , 1  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 1 , 1 , 1  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 1 , 1 , 1  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],

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

/// [`GagModeName::Cat`]
pub const CAT_GAGMODE: GagMode = GagMode {
    chars: ['m','r','e','o','a','u','w' , ' ',' ',' ',' ',' ',' ',' ',' ',' '],
    first: [ 4 , 2 , 1 , 0 , 1 , 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
    next: [
           [ 1 , 2 , 2 , 1 , 2 , 1 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 1 , 2 , 2 , 2 , 1 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 2 , 2 , 2 , 2 , 1  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 2 , 2 , 2 , 2 , 1  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 2 , 2 , 2 , 2 , 1  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 2 , 2 , 2 , 2 , 1  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 1 , 1 , 1 , 1 , 2  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],

           [ 0 , 0 , 0 , 0 , 0 , 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 0 , 0 , 0 , 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 0 , 0 , 0 , 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 0 , 0 , 0 , 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 0 , 0 , 0 , 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 0 , 0 , 0 , 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 0 , 0 , 0 , 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 0 , 0 , 0 , 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 0 , 0 , 0 , 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ]
    ]
};

/// [`GagModeName::Seal`]
pub const SEAL_GAGMODE: GagMode = GagMode {
    chars: ['g','i','h','p','b','e','a','f' , ' ',' ',' ',' ',' ',' ',' ',' '],
    first: [ 1 , 1 , 1 , 1 , 1 , 1 , 1 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
    next: [
           [ 1 , 1 , 1 , 0 , 0 , 4 , 2 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 1 , 2 , 2 , 1 , 2 , 1 , 1 , 1  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 1 , 1 , 1 , 2 , 1 , 0 , 0 , 2  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 1 , 4 , 0 , 1 , 0 , 0 , 2  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 1 , 1 , 0 , 1 , 1 , 0 , 2  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 2 , 1 , 1 , 0 , 0 , 0 , 2 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 1 , 2 , 2 , 1 , 1 , 1 , 0 , 4  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],

           [ 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0  ,  0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ]
    ]
};

/// [`GagModeName::Bee`].
pub const BEE_GAGMODE: GagMode = GagMode {
    chars: ['b','u','z',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' '],
    first: [ 4 , 0 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
    next: [
           [ 0 , 1 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
           [ 0 , 0 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ],
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
