//! A general-purpose genomics crate for dealing with DNA.

#![warn(missing_docs)]

use std::{convert::TryFrom, fmt::Display, str::FromStr};

// TODO: add a packed module with the PackedDna struct
//
// this struct must have the following:
// 1. A representation that is more memory efficient that simply storing a vector of `Nuc`
// 2. A FromStr implementation (should be case insensitive like the `Nuc` impl)
// 3. A `FromIterator` implementation to construct it from an iterator over `Nuc`s
// 4. A `fn get(&self, idx: usize) -> Nuc` getter for a particular nucleotide
//
// Make sure to unit test and document all elements
// Also, the internal representation of the PackedDna struct should be privately scoped

/// A nucleotide
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Nuc {
    /// Adenine
    A,
    /// Cytosine
    C,
    /// Guanine
    G,
    /// Thymine
    T,
}

/// An error that can occur when parsing a nucleotide.
#[derive(Debug, thiserror::Error)]
#[error("failed to parse nucleotide from {0}")]
pub struct ParseNucError<T: Display>(T);

impl TryFrom<char> for Nuc {
    type Error = ParseNucError<char>;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_uppercase() {
            'A' => Ok(Self::A),
            'C' => Ok(Self::C),
            'G' => Ok(Self::G),
            'T' => Ok(Self::T),
            _ => Err(ParseNucError(value)),
        }
    }
}

impl FromStr for Nuc {
    type Err = ParseNucError<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let upper = s.to_ascii_uppercase();
        match upper.as_str() {
            "A" => Ok(Self::A),
            "C" => Ok(Self::C),
            "G" => Ok(Self::G),
            "T" => Ok(Self::T),
            _ => Err(ParseNucError(upper)),
        }
    }
}

struct PackedDna {
    DNA: Vec<u8>,
}

impl FromStr for PackedDna {
    type Err = ParseNucError<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let upper = s.to_ascii_uppercase();
        let v: Vec<u8> = Vec::new();
        for c in upper.chars() {
            match c {
                'A' => v.push(0),
                'C' => v.push(1),
                'G' => v.push(2),
                'T' => v.push(3),
                _ => Err(ParseNucError(upper)),
            }
        }
        Ok(v)
    }
}

impl PackedDna {
    fn from_iterator(&self, iter: Vec<Nuc>) -> () {
        let v: Vec<u8> = Vec::new();
        for c in iter {
            match c {
                Nuc::A => v.push(0),
                Nuc::C => v.push(1),
                Nuc::G => v.push(2),
                Nuc::T => v.push(3),
            }
        }
        self.DNA = v
    }
}

impl PackedDna {
    fn get(&self, idx: usize) -> Nuc {
        match self.DNA[idx] {
            0 => Nuc::A,
            1 => Nuc::C,
            2 => Nuc::G,
            3 => Nuc::T, 
        }
    }
}

#[cfg(test)]
mod tests {
    // TODO: fill in tests
    use super::*;

    #[test]
    fn tryfrom_char_A() {
        let nucTryFromA = Nuc::try_from('A');
        match nucTryFromA {
            Ok(x) => assert_eq!(x, Nuc::A),
            Err(e) => println!(" {e:?} error is returned"),
        }
    }

    #[test]
    fn tryfrom_char_C() {
        let nucTryFromC = Nuc::try_from('C');
        match nucTryFromC {
            Ok(x) => assert_eq!(x, Nuc::C),
            Err(e) => println!(" {e:?} error is returned"),
        }
    }

    #[test]
    fn tryfrom_char_G() {
        let nucTryFromG = Nuc::try_from('G');
        match nucTryFromG {
            Ok(x) => assert_eq!(x, Nuc::G),
            Err(e) => println!(" {e:?} error is returned"),
        }
    }

    #[test]
    fn tryfrom_char_T() {
        let nucTryFromT = Nuc::try_from('T');
        match nucTryFromT {
            Ok(x) => assert_eq!(x, Nuc::T),
            Err(e) => println!(" {e:?} error is returned"),
        }
    }

    #[test]
    fn fromstr_a() {
        let nucFromStrA = Nuc::from_str("a");
        match nucFromStrA {
            Ok(x) => assert_eq!(x, Nuc::A),
            Err(e) => println!(" {e:?} error is returned"),
        }
    }

    #[test]
    fn fromstr_c() {
        let nucFromStrC = Nuc::from_str("c");
        match nucFromStrC {
            Ok(x) => assert_eq!(x, Nuc::C),
            Err(e) => println!(" {e:?} error is returned"),
        }
    }

    #[test]
    fn fromstr_g() {
        let nucFromStrG = Nuc::from_str("g");
        match nucFromStrG {
            Ok(x) => assert_eq!(x, Nuc::G),
            Err(e) => println!(" {e:?} error is returned"),
        }
    }

    #[test]
    fn fromstr_t() {
        let nucFromStrT = Nuc::from_str("t");
        match nucFromStrT {
            Ok(x) => assert_eq!(x, Nuc::T),
            Err(e) => println!(" {e:?} error is returned"),
        }
    }
}
